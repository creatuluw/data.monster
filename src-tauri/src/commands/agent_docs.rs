//! Agent docs (workspace-file-first FR-11): the app's documentation FOR coding agents,
//! shipped inside the binary and synced into the workspace as `README.md` + `dm/docs/`.
//!
//! Progressive disclosure: README (L0 entry, always read) → dm/docs/INDEX.md (L1
//! wayfinding: intent → file → format doc) → formats/*.md + concepts + recipes (L2,
//! self-contained, one annotated example each) → reference/ (L3 exhaustive).
//!
//! App-owned: regenerated whenever `dm/docs/.version` ≠ the app version (or missing).
//! Line budgets are enforced by the frontend test suite (tests/agent-docs.test.ts).

use std::fs;
use std::path::Path;

use crate::commands::dm_store;

const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

struct Doc {
    /// Workspace-relative destination (under `dm/docs/`, except README at root).
    dest: &'static str,
    body: &'static str,
}

const DOCS: &[Doc] = &[
    Doc { dest: "README.md", body: include_str!("../../agent-docs/README.md") },
    Doc { dest: "INDEX.md", body: include_str!("../../agent-docs/INDEX.md") },
    Doc { dest: "concepts.md", body: include_str!("../../agent-docs/concepts.md") },
    Doc { dest: "formats/page-doc.md", body: include_str!("../../agent-docs/formats/page-doc.md") },
    Doc { dest: "formats/master-items.md", body: include_str!("../../agent-docs/formats/master-items.md") },
    Doc { dest: "formats/saved-queries.md", body: include_str!("../../agent-docs/formats/saved-queries.md") },
    Doc { dest: "formats/connections.md", body: include_str!("../../agent-docs/formats/connections.md") },
    Doc { dest: "formats/relationships.md", body: include_str!("../../agent-docs/formats/relationships.md") },
    Doc { dest: "reference/page-doc-fields.md", body: include_str!("../../agent-docs/reference/page-doc-fields.md") },
    Doc { dest: "recipes/common-tasks.md", body: include_str!("../../agent-docs/recipes/common-tasks.md") },
];

/// Sync docs into the workspace: writes everything when `dm/docs/.version` is missing
/// or differs from the app version; no-op otherwise (user edits between versions win).
pub fn sync_agent_docs(ws: &Path) -> Result<(), String> {
    let marker = ws.join("dm").join("docs").join(".version");
    if let Ok(v) = fs::read_to_string(&marker) {
        if v.trim() == APP_VERSION {
            return Ok(());
        }
    }

    for doc in DOCS {
        let path = if doc.dest == "README.md" {
            ws.join("README.md")
        } else {
            ws.join("dm").join("docs").join(doc.dest)
        };
        // README at the workspace root: only seed when missing (users may customize it;
        // the canonical copy always lives in dm/docs/).
        if doc.dest == "README.md" && path.exists() {
            continue;
        }
        dm_store::atomic_write(&path, doc.body.as_bytes())?;
    }

    dm_store::atomic_write(&marker, format!("{APP_VERSION}\n").as_bytes())?;
    eprintln!("[agent-docs] synced dm/docs to app version {APP_VERSION}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp_ws(tag: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("docs-test-{}-{tag}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn writes_all_docs_and_version_marker() {
        let ws = tmp_ws("sync");
        sync_agent_docs(&ws).unwrap();
        assert!(ws.join("README.md").exists());
        for doc in DOCS {
            if doc.dest == "README.md" {
                continue;
            }
            let p = ws.join("dm").join("docs").join(doc.dest);
            assert!(p.exists(), "missing {}", doc.dest);
            assert_eq!(fs::read_to_string(p).unwrap(), doc.body);
        }
        let marker = fs::read_to_string(ws.join("dm/docs/.version")).unwrap();
        assert_eq!(marker.trim(), APP_VERSION);
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn same_version_is_noop_and_preserves_user_edits() {
        let ws = tmp_ws("noop");
        sync_agent_docs(&ws).unwrap();
        let index = ws.join("dm/docs/INDEX.md");
        fs::write(&index, "my customized index").unwrap();
        sync_agent_docs(&ws).unwrap(); // same version → untouched
        assert_eq!(fs::read_to_string(index).unwrap(), "my customized index");
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn version_change_regenerates() {
        let ws = tmp_ws("regen");
        sync_agent_docs(&ws).unwrap();
        fs::write(ws.join("dm/docs/.version"), "0.0.1\n").unwrap();
        fs::write(ws.join("dm/docs/INDEX.md"), "stale").unwrap();
        sync_agent_docs(&ws).unwrap();
        assert!(fs::read_to_string(ws.join("dm/docs/INDEX.md")).unwrap().starts_with("# INDEX"));
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn user_readme_at_root_is_never_overwritten() {
        let ws = tmp_ws("readme");
        fs::write(ws.join("README.md"), "my own readme").unwrap();
        sync_agent_docs(&ws).unwrap();
        assert_eq!(fs::read_to_string(ws.join("README.md")).unwrap(), "my own readme");
        // canonical copy still lands in dm/docs/
        assert!(ws.join("dm/docs").is_dir());
        let _ = fs::remove_dir_all(&ws);
    }
}
