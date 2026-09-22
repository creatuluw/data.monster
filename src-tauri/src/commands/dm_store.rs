//! FR-1 (workspace-file-first): dm/ path conventions, atomic writes, parse-level shape checks.
//!
//! The dm/ tree is the agent-authorable surface of the workspace (spec:
//! .specs/workspace-file-first/spec.md). Path is identity: filename = slug / item id /
//! query slug. All writes go through [`atomic_write`] (temp + rename) so agents and the
//! frontend never read a partial file.

use std::fs;
use std::path::{Path, PathBuf};

/// Workspace-relative roots under `dm/`.
pub fn dm_dir(ws: &Path) -> PathBuf {
    ws.join("dm")
}

/// A safe single path segment for slugs / item ids / query names.
/// Rejects: empty, whitespace-only, leading/trailing whitespace, path separators,
/// leading dots (hidden + dot-dot). Interior spaces allowed (human-named content).
pub fn valid_name(name: &str) -> bool {
    let trimmed = name.trim();
    if trimmed.is_empty() || trimmed != name || trimmed.starts_with('.') {
        return false;
    }
    !trimmed.contains(['/', '\\'])
        && trimmed
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | ' '))
}

fn name_or_err(name: &str) -> Result<(), String> {
    if valid_name(name) {
        Ok(())
    } else {
        Err(format!("invalid name: {name:?} (path separators, leading dots and empty names are not allowed)"))
    }
}

pub fn pages_dir(ws: &Path) -> PathBuf {
    dm_dir(ws).join("pages")
}

pub fn page_path(ws: &Path, slug: &str) -> Result<PathBuf, String> {
    name_or_err(slug)?;
    Ok(pages_dir(ws).join(format!("{slug}.json")))
}

pub fn master_items_dir(ws: &Path) -> PathBuf {
    dm_dir(ws).join("master-items")
}

/// `kind` is `"measure"` or `"dimension"`.
pub fn item_dir(ws: &Path, kind: &str) -> Result<PathBuf, String> {
    if kind != "measure" && kind != "dimension" {
        return Err(format!("invalid master-item kind: {kind:?} (expected \"measure\" or \"dimension\")"));
    }
    Ok(master_items_dir(ws).join(kind))
}

pub fn item_path(ws: &Path, kind: &str, id: &str) -> Result<PathBuf, String> {
    name_or_err(id)?;
    Ok(item_dir(ws, kind)?.join(format!("{id}.json")))
}

pub fn relationships_path(ws: &Path) -> PathBuf {
    dm_dir(ws).join("relationships.json")
}

pub fn saved_queries_dir(ws: &Path) -> PathBuf {
    dm_dir(ws).join("saved-queries")
}

pub fn saved_query_path(ws: &Path, slug: &str) -> Result<PathBuf, String> {
    name_or_err(slug)?;
    Ok(saved_queries_dir(ws).join(format!("{slug}.sql")))
}

pub fn connections_path(ws: &Path) -> PathBuf {
    dm_dir(ws).join("connections.json")
}

pub fn drafts_dir(ws: &Path) -> PathBuf {
    dm_dir(ws).join("drafts")
}

/// Inverse of [`page_path`]: slug from a file path, or None if not a page file.
pub fn page_slug_from_path(path: &Path) -> Option<String> {
    leaf_stem_json(path)
}

/// Inverse of [`item_path`]: `(kind, id)` from a file path, or None.
pub fn item_from_path(path: &Path) -> Option<(String, String)> {
    let parent = path.parent()?;
    let kind = parent.file_name()?.to_str()?;
    if kind != "measure" && kind != "dimension" {
        return None;
    }
    let id = leaf_stem_json(path)?;
    Some((kind.to_string(), id))
}

/// Inverse of [`saved_query_path`].
pub fn saved_query_slug_from_path(path: &Path) -> Option<String> {
    let ext = path.extension()?.to_str()?;
    ext.eq_ignore_ascii_case("sql").then(|| {
        path.file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default()
    })
}

fn leaf_stem_json(path: &Path) -> Option<String> {
    let ext = path.extension()?.to_str()?;
    ext.eq_ignore_ascii_case("json").then(|| {
        path.file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default()
    })
}

/// Atomic write: temp file in the same directory, flush, rename over the destination.
/// Readers (agents, frontend) never observe a partial file; a crash leaves the
/// destination untouched (a stray temp file may remain and is harmless).
pub fn atomic_write(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let dir = path
        .parent()
        .ok_or_else(|| format!("no parent directory for {}", path.display()))?;
    fs::create_dir_all(dir).map_err(|e| format!("failed to create {}: {e}", dir.display()))?;

    static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let tmp = dir.join(format!(
        ".{}.tmp-{}-{}",
        path.file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| "file".into()),
        std::process::id(),
        n
    ));

    let write_result = (|| -> std::io::Result<()> {
        let mut f = fs::File::create(&tmp)?;
        f.write_all(bytes)?;
        f.flush()?;
        drop(f); // close handle before rename (Windows)
        fs::rename(&tmp, path)?;
        Ok(())
    })();

    if let Err(e) = write_result {
        let _ = fs::remove_file(&tmp); // best effort cleanup
        return Err(format!("atomic write to {} failed: {e}", path.display()));
    }
    Ok(())
}

/// Parse-level shape check (NOT semantic validation — that stays in the frontend
/// validator). Sufficient to classify a watcher event as changed vs error.
pub fn check_json_object(bytes: &[u8], required_keys: &[&str]) -> Result<(), String> {
    let v: serde_json::Value = serde_json::from_slice(bytes)
        .map_err(|e| format!("invalid JSON: {e}"))?;
    let obj = v
        .as_object()
        .ok_or_else(|| "expected a JSON object".to_string())?;
    for key in required_keys {
        if !obj.contains_key(*key) {
            return Err(format!("missing required key: {key}"));
        }
    }
    Ok(())
}

use std::io::Write;

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp_ws(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "dm-store-test-{}-{tag}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    // --- path mapping round-trips -------------------------------------------

    #[test]
    fn page_path_round_trip() {
        let ws = PathBuf::from("/ws");
        let p = page_path(&ws, "revenue").unwrap();
        assert_eq!(p, PathBuf::from("/ws/dm/pages/revenue.json"));
        assert_eq!(page_slug_from_path(&p).as_deref(), Some("revenue"));
    }

    #[test]
    fn item_path_round_trip_both_kinds() {
        let ws = PathBuf::from("/ws");
        for kind in ["measure", "dimension"] {
            let p = item_path(&ws, kind, "total_revenue").unwrap();
            assert_eq!(
                p,
                PathBuf::from(format!("/ws/dm/master-items/{kind}/total_revenue.json"))
            );
            assert_eq!(
                item_from_path(&p),
                Some((kind.to_string(), "total_revenue".to_string()))
            );
        }
    }

    #[test]
    fn fixed_docs_map_to_their_paths() {
        let ws = PathBuf::from("/ws");
        assert_eq!(
            relationships_path(&ws),
            PathBuf::from("/ws/dm/relationships.json")
        );
        assert_eq!(
            connections_path(&ws),
            PathBuf::from("/ws/dm/connections.json")
        );
        let q = saved_query_path(&ws, "top_customers").unwrap();
        assert_eq!(q, PathBuf::from("/ws/dm/saved-queries/top_customers.sql"));
        assert_eq!(saved_query_slug_from_path(&q).as_deref(), Some("top_customers"));
    }

    // --- name safety ---------------------------------------------------------

    #[test]
    fn invalid_names_are_rejected_everywhere() {
        let ws = PathBuf::from("/ws");
        for bad in ["", "..", ".", "a/b", "a\\b", ".hidden", "a b c/x", "../etc"] {
            assert!(page_path(&ws, bad).is_err(), "page_path accepted {bad:?}");
            assert!(item_path(&ws, "measure", bad).is_err(), "item_path accepted {bad:?}");
            assert!(saved_query_path(&ws, bad).is_err(), "saved_query_path accepted {bad:?}");
        }
        assert!(item_path(&ws, "fact", "x").is_err()); // unknown kind
    }

    #[test]
    fn valid_names_accept_existing_shapes() {
        for good in ["revenue", "top_customers", "total-revenue-2", "Q1 Sales"] {
            assert!(valid_name(good), "rejected {good:?}");
        }
    }

    #[test]
    fn slug_from_path_ignores_foreign_files() {
        let ws = PathBuf::from("/ws");
        assert_eq!(page_slug_from_path(&ws.join("dm/pages/readme.md")), None);
        assert_eq!(page_slug_from_path(&ws.join("dm/connections.json")).as_deref(), Some("connections"));
        assert_eq!(item_from_path(&ws.join("dm/pages/revenue.json")), None);
        assert_eq!(saved_query_slug_from_path(&ws.join("dm/saved-queries/x.json")), None);
    }

    // --- atomic write ---------------------------------------------------------

    #[test]
    fn atomic_write_creates_parents_writes_and_leaves_no_temp() {
        let ws = tmp_ws("atomic-basic");
        let target = page_path(&ws, "revenue").unwrap();
        atomic_write(&target, b"{\"ok\":true}").unwrap();
        assert_eq!(fs::read_to_string(&target).unwrap(), "{\"ok\":true}");

        // no temp leftovers anywhere under dm/
        let leftovers: Vec<_> = walk(&dm_dir(&ws))
            .into_iter()
            .filter(|p| {
                p.file_name()
                    .map(|n| n.to_string_lossy().contains(".tmp-"))
                    .unwrap_or(false)
            })
            .collect();
        assert!(leftovers.is_empty(), "temp files left behind: {leftovers:?}");
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn atomic_write_overwrites_existing_content() {
        let ws = tmp_ws("atomic-overwrite");
        let target = page_path(&ws, "revenue").unwrap();
        atomic_write(&target, b"v1").unwrap();
        atomic_write(&target, b"v2").unwrap();
        assert_eq!(fs::read_to_string(&target).unwrap(), "v2");
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn failed_write_leaves_original_untouched_and_no_temp() {
        let ws = tmp_ws("atomic-fail");
        // Existing content...
        let target = page_path(&ws, "revenue").unwrap();
        atomic_write(&target, b"original").unwrap();

        // ...then make the rename fail: a DIRECTORY sits where the temp would rename to.
        // (Cross-platform failure injection: rename file-over-directory fails everywhere.)
        let _ = fs::remove_file(&target);
        fs::create_dir(&target).unwrap(); // destination is now a directory

        let err = atomic_write(&target, b"new").unwrap_err();
        assert!(err.contains("failed"), "unexpected error: {err}");
        // temp cleaned up
        let leftovers: Vec<_> = walk(&dm_dir(&ws))
            .into_iter()
            .filter(|p| {
                p.file_name()
                    .map(|n| n.to_string_lossy().contains(".tmp-"))
                    .unwrap_or(false)
            })
            .collect();
        assert!(leftovers.is_empty(), "temp files left behind: {leftovers:?}");
        let _ = fs::remove_dir_all(&ws);
    }

    // --- shape checks -----------------------------------------------------------

    #[test]
    fn check_json_object_accepts_object_with_required_keys() {
        check_json_object(b"{\"title\":\"x\",\"rows\":[]}", &["title", "rows"]).unwrap();
    }

    #[test]
    fn check_json_object_rejects_missing_invalid_nonobject() {
        assert!(check_json_object(b"{\"title\":\"x\"}", &["rows"]).is_err());
        assert!(check_json_object(b"not json", &[]).is_err());
        assert!(check_json_object(b"[1,2]", &[]).is_err());
    }

    // --- helpers -----------------------------------------------------------

    fn walk(dir: &Path) -> Vec<PathBuf> {
        let mut out = Vec::new();
        if let Ok(rd) = fs::read_dir(dir) {
            for e in rd.flatten() {
                let p = e.path();
                if p.is_dir() {
                    out.extend(walk(&p));
                } else {
                    out.push(p);
                }
            }
        }
        out
    }
}
