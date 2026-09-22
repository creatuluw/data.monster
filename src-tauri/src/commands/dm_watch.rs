//! dm/ file watcher (workspace-file-first FR-7): watches the workspace `dm/` tree and
//! turns file changes into Tauri events the frontend hot-reloads on.
//!
//! Pipeline: notify events → pending set → debounced flush (~300ms) → echo check
//! (skip the app's own atomic_writes) → classify + parse-level validate → emit
//! `dm:changed {kind, name, removed}` / `dm:error {path, reason}`.
//!
//! `classify` is pure (no AppHandle) so the classification matrix is unit-testable;
//! the thread wiring is thin. Echo suppression lives on the registry here and is fed
//! by `dm_store::atomic_write` (window ≥ write debounce, so self-writes never loop).

use notify::{RecursiveMode, Watcher};
use tauri::{Emitter, Manager};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{mpsc, Mutex, OnceLock};
use std::time::{Duration, Instant};

use crate::commands::{dm_store, saved_queries};

pub const DEBOUNCE_MS: u64 = 300;
pub const ECHO_WINDOW_MS: u64 = 600;

#[derive(Debug, PartialEq, Clone)]
pub enum DmEvent {
    Changed { kind: String, name: Option<String>, removed: bool },
    Error { path: String, reason: String },
}

// --- echo suppression --------------------------------------------------------

fn self_writes() -> &'static Mutex<HashMap<PathBuf, Instant>> {
    static REG: OnceLock<Mutex<HashMap<PathBuf, Instant>>> = OnceLock::new();
    REG.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Called by `dm_store::atomic_write` after a successful write: this path's watcher
/// events are ours — suppress them.
pub fn mark_self_write(path: &Path) {
    let mut reg = self_writes().lock().unwrap();
    reg.insert(path.to_path_buf(), Instant::now());
    // prune while we're here
    reg.retain(|_, t| t.elapsed() < Duration::from_millis(ECHO_WINDOW_MS));
}

pub(crate) fn is_self_write(path: &Path) -> bool {
    let reg = self_writes().lock().unwrap();
    reg.get(path).map(|t| t.elapsed() < Duration::from_millis(ECHO_WINDOW_MS)).unwrap_or(false)
}

// --- classification (pure) ---------------------------------------------------

/// Map a changed file under the workspace to a DmEvent. `None` = ignore (drafts,
/// docs, README, .env, settings, unknown files — the agent can scribble freely).
pub(crate) fn classify(ws: &Path, path: &Path, removed: bool) -> Option<DmEvent> {
    let rel = path.strip_prefix(ws).ok()?;
    let comps: Vec<&str> = rel
        .components()
        .map(|c| c.as_os_str().to_str())
        .collect::<Option<Vec<_>>>()?;

    let changed = |kind: &str, name: Option<String>| {
        Some(DmEvent::Changed { kind: kind.into(), name, removed })
    };
    let err = |reason: String| Some(DmEvent::Error { path: rel.display().to_string(), reason });

    match comps.as_slice() {
        ["dm", "pages", _f] => {
            let slug = dm_store::page_slug_from_path(path)?;
            if removed {
                return changed("page", Some(slug));
            }
            let bytes = std::fs::read(path).ok()?;
            match dm_store::check_json_object(&bytes, &["rows"]) {
                Ok(()) => changed("page", Some(slug)),
                Err(reason) => err(reason),
            }
        }
        ["dm", "master-items", folder @ ("measures" | "dimensions"), _f] => {
            let (kind, id) = dm_store::item_from_path(path)?;
            let _ = folder;
            if removed {
                return changed(&kind, Some(id));
            }
            let bytes = std::fs::read(path).ok()?;
            match dm_store::check_json_object(&bytes, &["tableName", "expr"]) {
                Ok(()) => changed(&kind, Some(id)),
                Err(reason) => err(reason),
            }
        }
        ["dm", "relationships.json"] => {
            if removed {
                return changed("relationships", None);
            }
            let bytes = std::fs::read(path).ok()?;
            match dm_store::check_json_object(&bytes, &["relationships"]) {
                Ok(()) => changed("relationships", None),
                Err(reason) => err(reason),
            }
        }
        ["dm", "saved-queries", _f] => {
            let slug = dm_store::saved_query_slug_from_path(path)?;
            if removed {
                return changed("saved-query", Some(slug));
            }
            let bytes = std::fs::read(path).ok()?;
            match saved_queries::check_query_file(&bytes, &slug) {
                Ok(()) => changed("saved-query", Some(slug)),
                Err(reason) => err(reason),
            }
        }
        ["dm", "connections.json"] => {
            if removed {
                return changed("connections", None);
            }
            let bytes = std::fs::read(path).ok()?;
            match dm_store::check_json_object(&bytes, &["connections"]) {
                Ok(()) => changed("connections", None),
                Err(reason) => err(reason),
            }
        }
        _ => None,
    }
}

// --- watcher lifecycle ---------------------------------------------------------

struct Active {
    _watcher: notify::RecommendedWatcher, // drop = stop watching
}

fn active() -> &'static Mutex<Vec<Active>> {
    static ACTIVE: OnceLock<Mutex<Vec<Active>>> = OnceLock::new();
    ACTIVE.get_or_init(|| Mutex::new(Vec::new()))
}

/// Start (or replace) the watcher for a workspace. Called on DuckDB init; a workspace
/// switch replaces the previous watcher (dropping it stops the thread).
pub fn start(ws: PathBuf, app: tauri::AppHandle) -> Result<(), String> {
    let (tx, rx) = mpsc::channel();
    let mut watcher =
        notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| {
            if let Ok(event) = res {
                for path in event.paths {
                    let _ = tx.send((path, matches!(event.kind, notify::EventKind::Remove(_))));
                }
            }
        })
        .map_err(|e| format!("failed to start dm/ watcher: {e}"))?;
    watcher
        .watch(&ws.join("dm"), RecursiveMode::Recursive)
        .map_err(|e| format!("failed to watch dm/: {e}"))?;

    // debounced flusher: collect raw paths, emit each path once per window
    std::thread::spawn(move || {
        let mut pending: HashMap<PathBuf, (Instant, bool)> = HashMap::new();
        loop {
            // block on the first event, then drain whatever arrives within the window
            if let Ok((path, removed)) = rx.recv() {
                pending.insert(path, (Instant::now(), removed));
            }
            let deadline = Instant::now() + Duration::from_millis(DEBOUNCE_MS);
            loop {
                let now = Instant::now();
                let timeout = deadline.checked_duration_since(now).unwrap_or(Duration::ZERO);
                match rx.recv_timeout(timeout) {
                    Ok((path, removed)) => {
                        // removals win over modifications within the same window
                        pending
                            .entry(path)
                            .and_modify(|e| e.1 = e.1 || removed)
                            .or_insert((Instant::now(), removed));
                    }
                    Err(mpsc::RecvTimeoutError::Timeout) => break,
                    Err(mpsc::RecvTimeoutError::Disconnected) => return, // watcher dropped
                }
            }

            for (path, (seen, removed)) in pending.drain() {
                if seen.elapsed() < Duration::from_millis(DEBOUNCE_MS) {
                    continue; // still settling — next tick picks it up
                }
                if is_self_write(&path) {
                    continue; // our own atomic_write — never loop
                }
                if let Some(event) = classify(&ws, &path, removed) {
                    match event {
                        DmEvent::Changed { kind, name, removed } => {
                            let _ = app.emit(
                                "dm:changed",
                                serde_json::json!({ "kind": kind, "name": name, "removed": removed }),
                            );
                        }
                        DmEvent::Error { path, reason } => {
                            let _ =
                                app.emit("dm:error", serde_json::json!({ "path": path, "reason": reason }));
                        }
                    }
                }
            }
        }
    });

    active().lock().unwrap().push(Active { _watcher: watcher });
    Ok(())
}

/// Watch `data/incoming/` (FR-13): dropped CSV/Parquet/JSON files are auto-ingested
/// with smart defaults (table name = filename stem), then moved into `data/main/`.
pub fn start_incoming(app: tauri::AppHandle, ws: PathBuf) -> Result<(), String> {
    let incoming = ws.join("data").join("incoming");
    std::fs::create_dir_all(&incoming)
        .map_err(|e| format!("failed to create data/incoming: {e}"))?;

    let (tx, rx) = mpsc::channel();
    let mut watcher = notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| {
        if let Ok(event) = res {
            for path in event.paths {
                let _ = tx.send(path);
            }
        }
    })
    .map_err(|e| format!("failed to start incoming watcher: {e}"))?;
    watcher
        .watch(&incoming, RecursiveMode::NonRecursive)
        .map_err(|e| format!("failed to watch data/incoming: {e}"))?;

    std::thread::spawn(move || {
        let mut pending: HashMap<PathBuf, Instant> = HashMap::new();
        loop {
            if let Ok(path) = rx.recv() {
                if path.is_file() {
                    pending.insert(path, Instant::now());
                }
            }
            let deadline = Instant::now() + Duration::from_millis(DEBOUNCE_MS);
            loop {
                let timeout = deadline.checked_duration_since(Instant::now()).unwrap_or(Duration::ZERO);
                match rx.recv_timeout(timeout) {
                    Ok(path) => {
                        if path.is_file() {
                            pending.insert(path, Instant::now());
                        }
                    }
                    Err(mpsc::RecvTimeoutError::Timeout) => break,
                    Err(mpsc::RecvTimeoutError::Disconnected) => return,
                }
            }

            for (path, seen) in pending.drain() {
                if seen.elapsed() < Duration::from_millis(DEBOUNCE_MS) {
                    continue; // still settling
                }
                if !path.exists() {
                    continue; // moved/deleted since the event
                }
                use crate::state::DuckDbState;
                let state = app.state::<DuckDbState>();
                let guard = state.conn.lock();
                let Some(conn) = guard.as_ref() else {
                    let _ = app.emit(
                        "dm:error",
                        serde_json::json!({ "path": path.display().to_string(), "reason": "database not ready" }),
                    );
                    continue;
                };
                match crate::commands::incoming::ingest_incoming_file(conn, &ws, &path) {
                    Ok(table) => {
                        let _ = app.emit(
                            "dm:changed",
                            serde_json::json!({ "kind": "table", "name": table, "removed": false }),
                        );
                    }
                    Err(reason) => {
                        let _ = app.emit(
                            "dm:error",
                            serde_json::json!({ "path": path.display().to_string(), "reason": reason }),
                        );
                    }
                }
            }
        }
    });

    active().lock().unwrap().push(Active { _watcher: watcher });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn tmp_ws(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("watch-test-{}-{tag}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn write_page(ws: &Path, slug: &str, content: &str) {
        let p = dm_store::page_path(ws, slug).unwrap();
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, content).unwrap();
    }

    #[test]
    fn valid_page_classifies_as_changed() {
        let ws = tmp_ws("page-ok");
        write_page(&ws, "revenue", "{\"title\":\"R\",\"rows\":[]}");
        let p = dm_store::page_path(&ws, "revenue").unwrap();
        assert_eq!(
            classify(&ws, &p, false),
            Some(DmEvent::Changed { kind: "page".into(), name: Some("revenue".into()), removed: false })
        );
        assert_eq!(
            classify(&ws, &p, true),
            Some(DmEvent::Changed { kind: "page".into(), name: Some("revenue".into()), removed: true })
        );
        let _ = std::fs::remove_dir_all(&ws);
    }

    #[test]
    fn invalid_page_classifies_as_error() {
        let ws = tmp_ws("page-bad");
        write_page(&ws, "broken", "{oops");
        let p = dm_store::page_path(&ws, "broken").unwrap();
        match classify(&ws, &p, false) {
            Some(DmEvent::Error { path, reason }) => {
                assert!(path.contains("broken.json"), "{path}");
                assert!(!reason.is_empty());
            }
            other => panic!("expected error, got {other:?}"),
        }
        let _ = std::fs::remove_dir_all(&ws);
    }

    #[test]
    fn items_relationships_queries_connections_classify() {
        let ws = tmp_ws("matrix");
        let m = dm_store::item_path(&ws, "measure", "total").unwrap();
        std::fs::create_dir_all(m.parent().unwrap()).unwrap();
        std::fs::write(&m, json!({"tableName":"t","expr":"1"}).to_string()).unwrap();
        assert_eq!(
            classify(&ws, &m, false),
            Some(DmEvent::Changed { kind: "measure".into(), name: Some("total".into()), removed: false })
        );

        let rp = dm_store::relationships_path(&ws);
        std::fs::write(&rp, json!({"relationships":[]}).to_string()).unwrap();
        assert!(matches!(classify(&ws, &rp, false), Some(DmEvent::Changed { kind, .. }) if kind == "relationships"));

        let sq = dm_store::saved_query_path(&ws, "top").unwrap();
        std::fs::create_dir_all(sq.parent().unwrap()).unwrap();
        std::fs::write(&sq, "SELECT 1").unwrap();
        assert!(matches!(classify(&ws, &sq, false), Some(DmEvent::Changed { kind, .. }) if kind == "saved-query"));

        let cp = dm_store::connections_path(&ws);
        std::fs::write(&cp, json!({"connections":[]}).to_string()).unwrap();
        assert!(matches!(classify(&ws, &cp, false), Some(DmEvent::Changed { kind, .. }) if kind == "connections"));
        let _ = std::fs::remove_dir_all(&ws);
    }

    #[test]
    fn ignored_paths_return_none() {
        let ws = tmp_ws("ignore");
        for rel in [
            "dm/drafts/notes.md",
            "dm/docs/INDEX.md",
            "README.md",
            ".env",
            "settings.json",
            "data/main/x.csv",
            "dm/random.txt",
        ] {
            let p = ws.join(rel);
            std::fs::create_dir_all(p.parent().unwrap()).unwrap();
            std::fs::write(&p, "x").unwrap();
            assert_eq!(classify(&ws, &p, false), None, "{rel} must be ignored");
        }
        // outside the workspace entirely
        assert_eq!(classify(&ws, Path::new("C:/elsewhere/dm/pages/x.json"), false), None);
        let _ = std::fs::remove_dir_all(&ws);
    }

    #[test]
    fn self_writes_are_suppressed_within_window() {
        let ws = tmp_ws("echo");
        let p = dm_store::page_path(&ws, "a").unwrap();
        assert!(!is_self_write(&p));
        mark_self_write(&p);
        assert!(is_self_write(&p));
        let _ = std::fs::remove_dir_all(&ws);
    }
}
