use serde_json::json;
use tauri::State;

use crate::state::DuckDbState;

#[tauri::command]
pub fn save_table_labels(
    table_name: String,
    tags: String,
    group: Option<String>,
    state: State<'_, DuckDbState>,
) -> Result<(), String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    conn.execute(
        "INSERT INTO d8a_monster_table_labels (table_name, tags, \"group\") VALUES (?, ?, ?)
         ON CONFLICT (table_name) DO UPDATE SET tags = EXCLUDED.tags, \"group\" = EXCLUDED.\"group\"",
        duckdb::params![&table_name, &tags, &group],
    )
    .map_err(|e| format!("Failed to save labels: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn get_table_labels(
    table_name: String,
    state: State<'_, DuckDbState>,
) -> Result<serde_json::Value, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    let result = conn.query_row(
        "SELECT tags, \"group\" FROM d8a_monster_table_labels WHERE table_name = ?",
        duckdb::params![&table_name],
        |row| {
            let tags_str: Option<String> = row.get(0)?;
            let group: Option<String> = row.get(1)?;
            Ok((tags_str, group))
        },
    );

    match result {
        Ok((tags_str, group)) => {
            let tags: Vec<String> = tags_str
                .unwrap_or_default()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            Ok(json!({
                "tableName": table_name,
                "tags": tags,
                "group": group
            }))
        }
        Err(_) => Ok(json!({
            "tableName": table_name,
            "tags": [],
            "group": null
        })),
    }
}

#[tauri::command]
pub fn get_all_tags(state: State<'_, DuckDbState>) -> Result<Vec<String>, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    let mut stmt = conn
        .prepare("SELECT tags FROM d8a_monster_table_labels WHERE tags IS NOT NULL AND tags != ''")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            let tags: String = row.get(0)?;
            Ok(tags)
        })
        .map_err(|e| e.to_string())?;

    let mut all_tags: Vec<String> = Vec::new();
    for row in rows {
        let tags_str = row.map_err(|e| e.to_string())?;
        for tag in tags_str.split(',') {
            let trimmed = tag.trim().to_string();
            if !trimmed.is_empty() && !all_tags.contains(&trimmed) {
                all_tags.push(trimmed);
            }
        }
    }

    all_tags.sort();
    Ok(all_tags)
}

#[tauri::command]
pub fn get_all_groups(state: State<'_, DuckDbState>) -> Result<Vec<String>, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    let mut stmt = conn
        .prepare(
            "SELECT DISTINCT \"group\" FROM d8a_monster_table_labels WHERE \"group\" IS NOT NULL AND \"group\" != '' ORDER BY \"group\"",
        )
        .map_err(|e| e.to_string())?;

    let groups: Vec<String> = stmt
        .query_map([], |row| row.get(0))
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(groups)
}

#[cfg(test)]
mod tests {
    use crate::commands::database::initialize_schema;
    use duckdb::Connection;

    fn setup() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        initialize_schema(&conn).unwrap();
        conn
    }

    #[test]
    fn test_save_and_get_labels() {
        let conn = setup();
        conn.execute(
            "INSERT INTO d8a_monster_table_labels (table_name, tags, \"group\") VALUES ('t1', 'sales,finance', 'reports')",
            [],
        ).unwrap();

        let result: (Option<String>, Option<String>) = conn
            .query_row(
                "SELECT tags, \"group\" FROM d8a_monster_table_labels WHERE table_name = 't1'",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();

        let tags: Vec<String> = result.0.unwrap()
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        assert_eq!(tags, vec!["sales", "finance"]);
        assert_eq!(result.1, Some("reports".to_string()));
    }

    #[test]
    fn test_get_labels_missing() {
        let conn = setup();
        let result = conn.query_row(
            "SELECT tags, \"group\" FROM d8a_monster_table_labels WHERE table_name = 'nope'",
            [],
            |row| Ok((row.get::<_, Option<String>>(0)?, row.get::<_, Option<String>>(1)?)),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_upsert_labels() {
        let conn = setup();
        conn.execute(
            "INSERT INTO d8a_monster_table_labels (table_name, tags, \"group\") VALUES ('t1', 'a,b', 'g1')
             ON CONFLICT (table_name) DO UPDATE SET tags = EXCLUDED.tags, \"group\" = EXCLUDED.\"group\"",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO d8a_monster_table_labels (table_name, tags, \"group\") VALUES ('t1', 'c,d', 'g2')
             ON CONFLICT (table_name) DO UPDATE SET tags = EXCLUDED.tags, \"group\" = EXCLUDED.\"group\"",
            [],
        ).unwrap();

        let tags: String = conn.query_row(
            "SELECT tags FROM d8a_monster_table_labels WHERE table_name = 't1'",
            [],
            |row| row.get(0),
        ).unwrap();
        assert_eq!(tags, "c,d");

        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM d8a_monster_table_labels",
            [],
            |row| row.get(0),
        ).unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_get_all_tags() {
        let conn = setup();
        conn.execute("INSERT INTO d8a_monster_table_labels (table_name, tags) VALUES ('t1', 'alpha,beta')", []).unwrap();
        conn.execute("INSERT INTO d8a_monster_table_labels (table_name, tags) VALUES ('t2', 'beta,gamma')", []).unwrap();

        let mut stmt = conn.prepare("SELECT tags FROM d8a_monster_table_labels WHERE tags IS NOT NULL AND tags != ''").unwrap();
        let mut all_tags: Vec<String> = Vec::new();
        for row in stmt.query_map([], |row| row.get::<_, String>(0)).unwrap() {
            let tags_str = row.unwrap();
            for tag in tags_str.split(',') {
                let trimmed = tag.trim().to_string();
                if !trimmed.is_empty() && !all_tags.contains(&trimmed) {
                    all_tags.push(trimmed);
                }
            }
        }
        all_tags.sort();
        assert_eq!(all_tags, vec!["alpha", "beta", "gamma"]);
    }

    #[test]
    fn test_get_all_groups() {
        let conn = setup();
        conn.execute("INSERT INTO d8a_monster_table_labels (table_name, tags, \"group\") VALUES ('t1', '', 'finance')", []).unwrap();
        conn.execute("INSERT INTO d8a_monster_table_labels (table_name, tags, \"group\") VALUES ('t2', '', 'sales')", []).unwrap();
        conn.execute("INSERT INTO d8a_monster_table_labels (table_name, tags, \"group\") VALUES ('t3', '', 'finance')", []).unwrap();

        let mut stmt = conn.prepare(
                "SELECT DISTINCT \"group\" FROM d8a_monster_table_labels WHERE \"group\" IS NOT NULL AND \"group\" != '' ORDER BY \"group\"",
            ).unwrap();
        let groups: Vec<String> = stmt
            .query_map([], |row| row.get(0))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();
        assert_eq!(groups, vec!["finance", "sales"]);
    }
}
