use duckdb::Connection;
use std::fs;
use tauri::{State, AppHandle, Manager};
use crate::state::DuckDbState;
use crate::utils::{cleanup_orphaned_metadata, register_table_metadata};
use crate::commands::udfs::reload_udfs_internal;

/// Initialize DuckDB with persistent storage
#[tauri::command]
pub fn initialize_duckdb(app: AppHandle, state: State<DuckDbState>) -> Result<String, String> {
    // Acquire lock first to prevent race conditions
    let mut state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    
    // Check if already initialized (while holding the lock)
    if state_conn.is_some() {
        return Ok("DuckDB already initialized".to_string());
    }
    
    // Get persistent database path
    let app_data_dir = app.path().app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    
    // Create app data directory if it doesn't exist
    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir)
            .map_err(|e| format!("Failed to create app data directory: {}", e))?;
    }
    
    let db_path = app_data_dir.join("warphead.duckdb");
    let wal_path = app_data_dir.join("warphead.duckdb.wal");
    
    // Try to open persistent database connection with retry logic for hot-reload
    let conn = {
        let max_retries = 10;
        let mut retry_count = 0;
        
        loop {
            match Connection::open(&db_path) {
                Ok(connection) => break connection,
                Err(e) => {
                    let error_msg = e.to_string();
                    
                    // Check if this is a file lock error (happens during hot-reload)
                    let is_lock_error = error_msg.contains("database is locked") ||
                                       error_msg.contains("unable to open database file") ||
                                       error_msg.contains("unable to get a read lock");
                    
                    // Check if this is a WAL corruption error
                    let is_wal_error = error_msg.contains("Failure while replaying WAL file") || 
                                      error_msg.contains("INTERNAL Error") ||
                                      error_msg.contains("DatabaseManager::GetDefaultDatabase");
                    
                    if is_lock_error {
                        retry_count += 1;
                        if retry_count <= max_retries {
                            println!("⚠️ Database is locked (attempt {}/{}) - waiting for old process to release...", retry_count, max_retries);
                            std::thread::sleep(std::time::Duration::from_millis(300 * retry_count as u64));
                            continue;
                        } else {
                            println!("❌ Database still locked after {} attempts", max_retries);
                            return Err(format!("Database is locked after {} retry attempts. Please close all instances and restart.", max_retries));
                        }
                    } else if is_wal_error {
                        println!("⚠️ Detected corrupted WAL file. Attempting recovery...");
                        
                        // Try to delete the corrupted WAL file
                        if wal_path.exists() {
                            match fs::remove_file(&wal_path) {
                                Ok(_) => {
                                    println!("✅ Removed corrupted WAL file");
                                    // Retry opening after WAL removal
                                    continue;
                                }
                                Err(remove_err) => {
                                    // Check if it's a file lock error (os error 32 on Windows)
                                    let is_lock_error = remove_err.raw_os_error() == Some(32) || 
                                                       remove_err.to_string().contains("being used by another process");
                                    
                                    if is_lock_error {
                                        retry_count += 1;
                                        if retry_count <= max_retries {
                                            println!("⚠️ WAL file is locked (attempt {}/{}) - waiting...", retry_count, max_retries);
                                            std::thread::sleep(std::time::Duration::from_millis(300 * retry_count as u64));
                                            continue;
                                        } else {
                                            println!("❌ WAL file still locked after {} attempts", max_retries);
                                            return Err(format!("WAL file is locked after {} retry attempts. Please close all instances and restart.", max_retries));
                                        }
                                    } else {
                                        // Different file system error
                                        return Err(format!("Failed to remove corrupted WAL file: {}", remove_err));
                                    }
                                }
                            }
                        } else {
                            // WAL doesn't exist but we got WAL error, try once more
                            continue;
                        }
                    } else {
                        // Different error, propagate it immediately
                        return Err(error_msg);
                    }
                }
            }
        }
    };
    
    // Initialize schema
    initialize_schema(&conn)?;
    
    // Install PostgreSQL extension
    let _ = conn.execute("INSTALL postgres", []);
    let _ = conn.execute("LOAD postgres", []);
    
    // Migrate existing tables
    migrate_existing_data(&conn)?;
    
    // Initialize system table entries
    initialize_system_tables(&conn)?;
    
    // Clean up orphaned metadata
    let cleanup_result = cleanup_orphaned_metadata(&conn);
    match cleanup_result {
        Ok(msg) => eprintln!("Metadata cleanup: {}", msg),
        Err(e) => eprintln!("Warning: Metadata cleanup failed: {}", e),
    }
    
    // Reload UDFs after database initialization
    let udf_reload_result = reload_udfs_internal(&conn);
    match udf_reload_result {
        Ok(msg) => eprintln!("UDF reload: {}", msg),
        Err(e) => eprintln!("Warning: Failed to reload UDFs: {}", e),
    }
    
    // Initialize built-in aggregation functions as UDFs for user reference
    let builtin_funcs_result = initialize_builtin_functions(&conn);
    if let Err(e) = builtin_funcs_result {
        eprintln!("Warning: Failed to initialize built-in functions: {}", e);
    }
    
    // Initialize built-in chart templates from library
    let builtin_charts_result = initialize_builtin_charts(&conn);
    if let Err(e) = builtin_charts_result {
        eprintln!("Warning: Failed to initialize built-in chart templates: {}", e);
    }
    
    // Initialize built-in component templates from library
    let builtin_components_result = initialize_builtin_components(&conn);
    if let Err(e) = builtin_components_result {
        eprintln!("Warning: Failed to initialize built-in component templates: {}", e);
    }
    
    // Initialize built-in metrics from library
    let builtin_metrics_result = initialize_builtin_metrics(&conn);
    if let Err(e) = builtin_metrics_result {
        eprintln!("Warning: Failed to initialize built-in metrics: {}", e);
    }
    
    // Initialize built-in dimensions from library
    let builtin_dimensions_result = initialize_builtin_dimensions(&conn);
    if let Err(e) = builtin_dimensions_result {
        eprintln!("Warning: Failed to initialize built-in dimensions: {}", e);
    }
    
    // Check if sample data should be loaded before releasing the connection
    // This checks for a special marker in the system tables
    // The setting is controlled from the frontend via the warphead settings page
    let should_load_sample_data = check_should_load_sample_data(&conn);
    
    // Store the connection in state BEFORE calling other functions that need the lock
    *state_conn = Some(conn);
    drop(state_conn); // Release the lock
    
    // Load tables with 'on_app_load' ingestion strategy
    eprintln!("🔄 Loading tables with 'on_app_load' strategy...");
    let load_result = crate::commands::ingestion::load_on_app_load_tables(state.clone());
    match load_result {
        Ok(msg) => eprintln!("✅ {}", msg),
        Err(e) => eprintln!("⚠️  Warning: Failed to load on_app_load tables: {}", e),
    }
    
    if should_load_sample_data {
        // Initialize sample data on first run (needs to be async for remote loading)
        // Need to get the connection again for cloning
        let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
        let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
        
        let conn_clone = conn.try_clone()
            .map_err(|e| format!("Failed to clone connection for init: {}", e))?;
        
        // Release lock before long-running operation
        drop(state_conn);
        
        // IMPORTANT: Reload UDFs on the cloned connection since they're TEMP macros
        eprintln!("🔄 Reloading UDFs on cloned connection for sample data initialization...");
        let udf_reload_result = reload_udfs_internal(&conn_clone);
        match udf_reload_result {
            Ok(msg) => eprintln!("UDF reload on clone: {}", msg),
            Err(e) => eprintln!("Warning: Failed to reload UDFs on clone: {}", e),
        }
        
        let app_clone = app.clone();
        
        let sample_data_result = tauri::async_runtime::block_on(async move {
            crate::commands::init_data::initialize_sample_data(&conn_clone, &app_clone).await
        });
        
        match sample_data_result {
            Ok(msg) => eprintln!("Sample data: {}", msg),
            Err(e) => eprintln!("Warning: Failed to initialize sample data: {}", e),
        }
    } else {
        eprintln!("⏭️  Skipping sample data initialization (disabled in settings)");
    }
    
    Ok("DuckDB initialized successfully with persistent storage".to_string())
}

/// Check if sample data should be loaded based on settings
/// Returns true by default (for backwards compatibility and first-time users)
fn check_should_load_sample_data(conn: &Connection) -> bool {
    // Check if there's a setting stored in metadata
    // We store settings in the _warphead_table_metadata table with a special 'setting' type
    let check_query = "SELECT creation_query FROM _warphead_table_metadata WHERE table_name = '_setting_load_sample_data' AND table_type = 'setting'";
    
    match conn.query_row(check_query, [], |row| row.get::<_, String>(0)) {
        Ok(value) => {
            // The value is stored as 'true' or 'false' string
            value == "true"
        }
        Err(_) => {
            // No setting found, default to true (load sample data by default)
            // This ensures backwards compatibility and helps new users get started
            true
        }
    }
}

/// Initialize database schema (create system tables)
fn initialize_schema(conn: &Connection) -> Result<(), String> {
    // Create datamodels table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS datamodels (
            table_name VARCHAR PRIMARY KEY,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            description VARCHAR
        )",
        []
    ).map_err(|e| format!("Failed to create datamodels table: {}", e))?;
    
    // Create _warphead_table_metadata table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS _warphead_table_metadata (
            table_name VARCHAR PRIMARY KEY,
            table_type VARCHAR NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            creation_query TEXT
        )",
        []
    ).map_err(|e| format!("Failed to create _warphead_table_metadata table: {}", e))?;
    
    // Add creation_query column if it doesn't exist
    let _ = conn.execute(
        "ALTER TABLE _warphead_table_metadata ADD COLUMN IF NOT EXISTS creation_query TEXT",
        []
    );
    
    // Add ingestion-related columns
    let _ = conn.execute(
        "ALTER TABLE _warphead_table_metadata ADD COLUMN IF NOT EXISTS ingestion_strategy VARCHAR DEFAULT 'manual'",
        []
    );
    let _ = conn.execute(
        "ALTER TABLE _warphead_table_metadata ADD COLUMN IF NOT EXISTS is_ingested BOOLEAN DEFAULT false",
        []
    );
    let _ = conn.execute(
        "ALTER TABLE _warphead_table_metadata ADD COLUMN IF NOT EXISTS source_path TEXT",
        []
    );
    let _ = conn.execute(
        "ALTER TABLE _warphead_table_metadata ADD COLUMN IF NOT EXISTS source_type VARCHAR",
        []
    );
    let _ = conn.execute(
        "ALTER TABLE _warphead_table_metadata ADD COLUMN IF NOT EXISTS ingested_at TIMESTAMP",
        []
    );
    let _ = conn.execute(
        "ALTER TABLE _warphead_table_metadata ADD COLUMN IF NOT EXISTS ingested_row_count INTEGER",
        []
    );
    
    // Drop old relationships table if it exists (migration)
    let _ = conn.execute("DROP TABLE IF EXISTS _warphead_relationships", []);
    
    // Create _warphead_relationships table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS _warphead_relationships (
            from_table VARCHAR NOT NULL,
            from_column VARCHAR NOT NULL,
            to_table VARCHAR NOT NULL,
            to_column VARCHAR NOT NULL,
            relationship_type VARCHAR NOT NULL DEFAULT 'inferred',
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            PRIMARY KEY(from_table, from_column, to_table, to_column)
        )",
        []
    ).map_err(|e| format!("Failed to create _warphead_relationships table: {}", e))?;
    
    // Migrate old _warphead_metrics table schema
    let needs_migration = {
        let check_old_schema = conn.prepare("SELECT column_name FROM information_schema.columns WHERE table_name = '_warphead_metrics' AND column_name = 'format_string'");
        let has_format_string = check_old_schema.is_ok() && check_old_schema.unwrap().exists([]).unwrap_or(false);
        
        let check_source_table = conn.prepare("SELECT column_name FROM information_schema.columns WHERE table_name = '_warphead_metrics' AND column_name = 'source_table'");
        let has_source_table = check_source_table.is_ok() && check_source_table.unwrap().exists([]).unwrap_or(false);
        
        let check_slug = conn.prepare("SELECT column_name FROM information_schema.columns WHERE table_name = '_warphead_metrics' AND column_name = 'slug'");
        let has_slug = check_slug.is_ok() && check_slug.unwrap().exists([]).unwrap_or(false);
        
        has_format_string || !has_source_table || !has_slug
    };
    
    if needs_migration {
        let _ = conn.execute("DROP TABLE IF EXISTS _warphead_metrics", []);
    }
    
    // Create _warphead_metrics table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS _warphead_metrics (
            slug VARCHAR PRIMARY KEY,
            metric_name VARCHAR NOT NULL UNIQUE,
            formula TEXT NOT NULL,
            source_table VARCHAR NOT NULL,
            description TEXT,
            tags TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
        []
    ).map_err(|e| format!("Failed to create _warphead_metrics table: {}", e))?;
    
    // Migrate old _warphead_dimensions table
    let needs_dimension_migration = {
        let check_slug = conn.prepare("SELECT column_name FROM information_schema.columns WHERE table_name = '_warphead_dimensions' AND column_name = 'slug'");
        let has_slug = check_slug.is_ok() && check_slug.unwrap().exists([]).unwrap_or(false);
        !has_slug
    };
    
    if needs_dimension_migration {
        let _ = conn.execute("DROP TABLE IF EXISTS _warphead_dimensions", []);
    }
    
    // Create dimensions metadata table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS _warphead_dimensions (
            slug VARCHAR PRIMARY KEY,
            dimension_name VARCHAR NOT NULL UNIQUE,
            field_name VARCHAR NOT NULL,
            source_table VARCHAR NOT NULL,
            description TEXT,
            tags TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
        []
    ).map_err(|e| format!("Failed to create _warphead_dimensions table: {}", e))?;
    
    // PostgreSQL connection tables
    conn.execute(
        "CREATE TABLE IF NOT EXISTS _warphead_connections (
            connection_id VARCHAR PRIMARY KEY,
            connection_name VARCHAR NOT NULL UNIQUE,
            connection_type VARCHAR NOT NULL,
            secret_name VARCHAR NOT NULL,
            is_attached BOOLEAN DEFAULT false,
            attach_mode VARCHAR DEFAULT 'hybrid',
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            last_used_at TIMESTAMP,
            default_refresh_strategy VARCHAR DEFAULT 'on-demand',
            default_staleness_threshold_minutes INTEGER DEFAULT 60,
            auto_refresh_enabled BOOLEAN DEFAULT false,
            metadata JSON
        )",
        []
    ).map_err(|e| format!("Failed to create _warphead_connections table: {}", e))?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS _warphead_attached_tables (
            table_id VARCHAR PRIMARY KEY,
            connection_id VARCHAR NOT NULL,
            schema_name VARCHAR NOT NULL,
            table_name VARCHAR NOT NULL,
            full_name VARCHAR NOT NULL,
            access_mode VARCHAR NOT NULL,
            local_table_name VARCHAR,
            cached_at TIMESTAMP,
            cache_row_count INTEGER,
            cache_size_bytes INTEGER,
            refresh_strategy VARCHAR DEFAULT 'on-demand',
            staleness_threshold_minutes INTEGER,
            last_refresh_attempt TIMESTAMP,
            refresh_status VARCHAR DEFAULT 'idle',
            metadata JSON
        )",
        []
    ).map_err(|e| format!("Failed to create _warphead_attached_tables table: {}", e))?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS _warphead_import_log (
            import_id VARCHAR PRIMARY KEY,
            connection_id VARCHAR NOT NULL,
            source_table VARCHAR NOT NULL,
            target_table VARCHAR NOT NULL,
            rows_imported INTEGER,
            import_duration_ms INTEGER,
            filter_clause VARCHAR,
            status VARCHAR NOT NULL,
            error_message VARCHAR,
            imported_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
        []
    ).map_err(|e| format!("Failed to create _warphead_import_log table: {}", e))?;
    
    // Create chart templates table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS _warphead_chart_templates (
            slug VARCHAR PRIMARY KEY,
            chart_name VARCHAR NOT NULL UNIQUE,
            chart_type VARCHAR NOT NULL,
            chart_code TEXT NOT NULL,
            config_schema TEXT,
            description TEXT,
            tags TEXT,
            thumbnail_url TEXT,
            metrics TEXT,
            dimensions TEXT,
            sample_data TEXT,
            min_metrics INTEGER DEFAULT 1,
            max_metrics INTEGER DEFAULT 1,
            min_dimensions INTEGER DEFAULT 1,
            max_dimensions INTEGER DEFAULT 1,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
        []
    ).map_err(|e| format!("Failed to create _warphead_chart_templates table: {}", e))?;
    
    // Add min/max columns if they don't exist (migration for existing databases)
    let _ = conn.execute(
        "ALTER TABLE _warphead_chart_templates ADD COLUMN IF NOT EXISTS min_metrics INTEGER DEFAULT 1",
        []
    );
    let _ = conn.execute(
        "ALTER TABLE _warphead_chart_templates ADD COLUMN IF NOT EXISTS max_metrics INTEGER DEFAULT 1",
        []
    );
    let _ = conn.execute(
        "ALTER TABLE _warphead_chart_templates ADD COLUMN IF NOT EXISTS min_dimensions INTEGER DEFAULT 1",
        []
    );
    let _ = conn.execute(
        "ALTER TABLE _warphead_chart_templates ADD COLUMN IF NOT EXISTS max_dimensions INTEGER DEFAULT 1",
        []
    );
    
    // Create component templates table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS _warphead_component_templates (
            slug VARCHAR PRIMARY KEY,
            component_name VARCHAR NOT NULL UNIQUE,
            component_type VARCHAR NOT NULL,
            html_code TEXT NOT NULL,
            css_code TEXT,
            js_code TEXT,
            config_schema TEXT,
            description TEXT,
            tags TEXT,
            metrics TEXT,
            dimensions TEXT,
            sample_data TEXT,
            min_metrics INTEGER DEFAULT 0,
            max_metrics INTEGER DEFAULT 0,
            min_dimensions INTEGER DEFAULT 0,
            max_dimensions INTEGER DEFAULT 0,
            frameworks TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
        []
    ).map_err(|e| format!("Failed to create _warphead_component_templates table: {}", e))?;
    
    // Create saved queries table
    crate::commands::saved_queries::init_saved_queries_table(conn)?;
    
    Ok(())
}

/// Migrate existing tables from old datamodels table
fn migrate_existing_data(conn: &Connection) -> Result<(), String> {
    let migration_result = conn.execute(
        "INSERT INTO _warphead_table_metadata (table_name, table_type, created_at)
         SELECT table_name, 'model' as table_type, created_at 
         FROM datamodels 
         WHERE table_name NOT IN (SELECT table_name FROM _warphead_table_metadata)
         ON CONFLICT DO NOTHING",
        []
    );
    
    match migration_result {
        Ok(count) if count > 0 => {
            eprintln!("Migrated {} tables from datamodels to _warphead_table_metadata", count);
        },
        Err(e) => {
            eprintln!("Warning: Failed to migrate datamodels: {}", e);
        },
        _ => {}
    }
    
    Ok(())
}

/// Initialize system table entries
fn initialize_system_tables(conn: &Connection) -> Result<(), String> {
    let _ = register_table_metadata(conn, "datamodels", "system");
    let _ = register_table_metadata(conn, "_warphead_table_metadata", "system");
    let _ = register_table_metadata(conn, "_warphead_relationships", "system");
    let _ = register_table_metadata(conn, "_warphead_metrics", "system");
    Ok(())
}

/// Initialize built-in library functions from JSON file
fn initialize_builtin_functions(conn: &Connection) -> Result<(), String> {
    use duckdb::params;
    use crate::models::LibraryFunctionsJson;
    use std::path::PathBuf;
    
    // Create the UDF table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS _warphead_udfs (
            function_name TEXT PRIMARY KEY,
            parameters TEXT NOT NULL,
            return_type TEXT NOT NULL,
            function_body TEXT NOT NULL,
            description TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
        []
    ).map_err(|e| format!("Failed to create UDF table: {}", e))?;
    
    // Try to read library functions from runtime file first, fallback to embedded
    let json_content = {
        // Try runtime path (for development)
        let runtime_path = PathBuf::from("src/lib/udfs/library-functions.json");
        if runtime_path.exists() {
            eprintln!("Loading library functions from runtime file: {:?}", runtime_path);
            std::fs::read_to_string(&runtime_path)
                .unwrap_or_else(|_| include_str!("../../../src/lib/udfs/library-functions.json").to_string())
        } else {
            // Fallback to embedded version (for production)
            eprintln!("Loading library functions from embedded file");
            include_str!("../../../src/lib/udfs/library-functions.json").to_string()
        }
    };
    
    let library_functions: LibraryFunctionsJson = serde_json::from_str(&json_content)
        .map_err(|e| format!("Failed to parse library-functions.json: {}", e))?;
    
    let function_count = library_functions.functions.len();
    
    // Insert each library function
    let mut created_count = 0;
    let mut updated_count = 0;
    
    for func in library_functions.functions {
        // Check if function exists
        let exists: i64 = conn.query_row(
            "SELECT COUNT(*) FROM _warphead_udfs WHERE function_name = ?",
            params![&func.function_name],
            |row| row.get(0)
        ).unwrap_or(0);
        
        // Insert or update library functions from JSON (allows updating when JSON changes)
        conn.execute(
            "INSERT INTO _warphead_udfs (function_name, parameters, return_type, function_body, description, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, now(), now())
             ON CONFLICT (function_name) DO UPDATE SET 
                parameters = EXCLUDED.parameters,
                return_type = EXCLUDED.return_type,
                function_body = EXCLUDED.function_body,
                description = EXCLUDED.description,
                updated_at = now()",
            params![
                &func.function_name,
                &func.parameters,
                &func.return_type,
                &func.function_body,
                &func.description
            ]
        ).map_err(|e| format!("Failed to insert library UDF {}: {}", func.function_name, e))?;
        
        // Create the actual DuckDB macro (TEMP so it doesn't require storage v1.4.0)
        let create_query = format!(
            "CREATE OR REPLACE TEMP MACRO {}({}) AS {}",
            func.function_name,
            func.parameters,
            func.function_body
        );
        
        conn.execute(&create_query, [])
            .map_err(|e| format!("Failed to create macro for {}: {}", func.function_name, e))?;
        
        if exists > 0 {
            updated_count += 1;
        } else {
            created_count += 1;
        }
    }
    
    eprintln!("Loaded {} library functions from JSON ({} created, {} updated)", 
              function_count, created_count, updated_count);
    
    Ok(())
}

/// Initialize built-in library chart templates from JSON file
fn initialize_builtin_charts(conn: &Connection) -> Result<(), String> {
    use duckdb::params;
    use crate::models::LibraryChartsJson;
    use std::path::PathBuf;
    
    // Chart templates table is already created in initialize_schema
    // Just need to load the library charts
    
    // Try to read library charts from runtime file first, fallback to embedded
    let json_content = {
        // Try runtime path (for development)
        let runtime_path = PathBuf::from("src/lib/charts/library-charts.json");
        if runtime_path.exists() {
            eprintln!("Loading library charts from runtime file: {:?}", runtime_path);
            std::fs::read_to_string(&runtime_path)
                .unwrap_or_else(|_| include_str!("../../../src/lib/charts/library-charts.json").to_string())
        } else {
            // Fallback to embedded version (for production)
            eprintln!("Loading library charts from embedded file");
            include_str!("../../../src/lib/charts/library-charts.json").to_string()
        }
    };
    
    let library_charts: LibraryChartsJson = serde_json::from_str(&json_content)
        .map_err(|e| format!("Failed to parse library-charts.json: {}", e))?;
    
    let chart_count = library_charts.charts.len();
    
    // Insert each library chart
    let mut created_count = 0;
    let mut updated_count = 0;
    
    for chart in library_charts.charts {
        // Generate slug from chart_name
        let slug = chart.chart_name.to_lowercase()
            .trim()
            .replace(|c: char| !c.is_alphanumeric() && c != '-' && c != '_', "-")
            .replace("--", "-")
            .trim_matches('-')
            .to_string();
        
        // Check if chart template exists
        let exists: i64 = conn.query_row(
            "SELECT COUNT(*) FROM _warphead_chart_templates WHERE slug = ?",
            params![&slug],
            |row| row.get(0)
        ).unwrap_or(0);
        
        // Insert or update library charts from JSON (allows updating when JSON changes)
        conn.execute(
            "INSERT INTO _warphead_chart_templates 
             (slug, chart_name, chart_type, chart_code, config_schema, description, tags, metrics, dimensions, sample_data, min_metrics, max_metrics, min_dimensions, max_dimensions, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, now(), now())
             ON CONFLICT (slug) DO UPDATE SET 
                chart_name = EXCLUDED.chart_name,
                chart_type = EXCLUDED.chart_type,
                chart_code = EXCLUDED.chart_code,
                config_schema = EXCLUDED.config_schema,
                description = EXCLUDED.description,
                tags = EXCLUDED.tags,
                metrics = EXCLUDED.metrics,
                dimensions = EXCLUDED.dimensions,
                sample_data = EXCLUDED.sample_data,
                min_metrics = EXCLUDED.min_metrics,
                max_metrics = EXCLUDED.max_metrics,
                min_dimensions = EXCLUDED.min_dimensions,
                max_dimensions = EXCLUDED.max_dimensions,
                updated_at = now()",
            params![
                &slug,
                &chart.chart_name,
                &chart.chart_type,
                &chart.chart_code,
                &chart.config_schema.unwrap_or_default(),
                &chart.description,
                &chart.tags,
                &chart.metrics.unwrap_or_default(),
                &chart.dimensions.unwrap_or_default(),
                &chart.sample_data.unwrap_or_default(),
                &chart.min_metrics.unwrap_or(1),
                &chart.max_metrics.unwrap_or(1),
                &chart.min_dimensions.unwrap_or(1),
                &chart.max_dimensions.unwrap_or(1)
            ]
        ).map_err(|e| format!("Failed to insert library chart {}: {}", chart.chart_name, e))?;
        
        if exists > 0 {
            updated_count += 1;
        } else {
            created_count += 1;
        }
    }
    
    eprintln!("Loaded {} library chart templates from JSON ({} created, {} updated)", 
              chart_count, created_count, updated_count);
    
    Ok(())
}

/// Initialize built-in library component templates from JSON file
fn initialize_builtin_components(conn: &Connection) -> Result<(), String> {
    use duckdb::params;
    use crate::models::LibraryComponentsJson;
    use std::path::PathBuf;
    
    // Try to read library components from runtime file first, fallback to embedded
    let json_content = {
        let runtime_path = PathBuf::from("src/lib/ui/library-components.json");
        if runtime_path.exists() {
            eprintln!("Loading library components from runtime file: {:?}", runtime_path);
            std::fs::read_to_string(&runtime_path)
                .unwrap_or_else(|_| include_str!("../../../src/lib/ui/library-components.json").to_string())
        } else {
            eprintln!("Loading library components from embedded file");
            include_str!("../../../src/lib/ui/library-components.json").to_string()
        }
    };
    
    let library_components: LibraryComponentsJson = serde_json::from_str(&json_content)
        .map_err(|e| format!("Failed to parse library-components.json: {}", e))?;
    
    let component_count = library_components.components.len();
    let mut created_count = 0;
    let mut updated_count = 0;
    
    for component in library_components.components {
        let slug = component.component_name.to_lowercase()
            .trim()
            .replace(|c: char| !c.is_alphanumeric() && c != '-' && c != '_', "-")
            .replace("--", "-")
            .trim_matches('-')
            .to_string();
        
        let exists: i64 = conn.query_row(
            "SELECT COUNT(*) FROM _warphead_component_templates WHERE slug = ?",
            params![&slug],
            |row| row.get(0)
        ).unwrap_or(0);
        
        conn.execute(
            "INSERT INTO _warphead_component_templates 
             (slug, component_name, component_type, html_code, css_code, js_code, frameworks,
              config_schema, description, tags, metrics, dimensions, sample_data, 
              min_metrics, max_metrics, min_dimensions, max_dimensions, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, now(), now())
             ON CONFLICT (slug) DO UPDATE SET 
                component_name = EXCLUDED.component_name,
                component_type = EXCLUDED.component_type,
                html_code = EXCLUDED.html_code,
                css_code = EXCLUDED.css_code,
                js_code = EXCLUDED.js_code,
                frameworks = EXCLUDED.frameworks,
                config_schema = EXCLUDED.config_schema,
                description = EXCLUDED.description,
                tags = EXCLUDED.tags,
                metrics = EXCLUDED.metrics,
                dimensions = EXCLUDED.dimensions,
                sample_data = EXCLUDED.sample_data,
                min_metrics = EXCLUDED.min_metrics,
                max_metrics = EXCLUDED.max_metrics,
                min_dimensions = EXCLUDED.min_dimensions,
                max_dimensions = EXCLUDED.max_dimensions,
                updated_at = now()",
            params![
                &slug,
                &component.component_name,
                &component.component_type,
                &component.html_code,
                &component.css_code.unwrap_or_default(),
                &component.js_code.unwrap_or_default(),
                &component.frameworks.unwrap_or_default(),
                &component.config_schema.unwrap_or_default(),
                &component.description,
                &component.tags,
                &component.metrics.unwrap_or_default(),
                &component.dimensions.unwrap_or_default(),
                &component.sample_data.unwrap_or_default(),
                &component.min_metrics.unwrap_or(0),
                &component.max_metrics.unwrap_or(0),
                &component.min_dimensions.unwrap_or(0),
                &component.max_dimensions.unwrap_or(0)
            ]
        ).map_err(|e| format!("Failed to insert library component {}: {}", component.component_name, e))?;
        
        if exists > 0 {
            updated_count += 1;
        } else {
            created_count += 1;
        }
    }
    
    eprintln!("Loaded {} library component templates from JSON ({} created, {} updated)", 
              component_count, created_count, updated_count);
    
    Ok(())
}

/// Initialize built-in library metrics from JSON file
fn initialize_builtin_metrics(conn: &Connection) -> Result<(), String> {
    use duckdb::params;
    use crate::models::LibraryMetricsJson;
    use std::path::PathBuf;
    
    // Metrics table is already created in initialize_schema
    // Just need to load the library metrics
    
    // Try to read library metrics from runtime file first, fallback to embedded
    let json_content = {
        // Try runtime path (for development)
        let runtime_path = PathBuf::from("src/lib/on_init/metrics.json");
        if runtime_path.exists() {
            eprintln!("Loading library metrics from runtime file: {:?}", runtime_path);
            std::fs::read_to_string(&runtime_path)
                .unwrap_or_else(|_| include_str!("../../../src/lib/on_init/metrics.json").to_string())
        } else {
            // Fallback to embedded version (for production)
            eprintln!("Loading library metrics from embedded file");
            include_str!("../../../src/lib/on_init/metrics.json").to_string()
        }
    };
    
    let library_metrics: LibraryMetricsJson = serde_json::from_str(&json_content)
        .map_err(|e| format!("Failed to parse metrics.json: {}", e))?;
    
    let metric_count = library_metrics.metrics.len();
    
    // Insert each library metric
    let mut created_count = 0;
    let mut updated_count = 0;
    
    for metric in library_metrics.metrics {
        // Check if metric exists
        let exists: i64 = conn.query_row(
            "SELECT COUNT(*) FROM _warphead_metrics WHERE slug = ?",
            params![&metric.slug],
            |row| row.get(0)
        ).unwrap_or(0);
        
        // Insert or update library metrics from JSON (allows updating when JSON changes)
        conn.execute(
            "INSERT INTO _warphead_metrics 
             (slug, metric_name, formula, source_table, description, tags, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, now(), now())
             ON CONFLICT (slug) DO UPDATE SET 
                metric_name = EXCLUDED.metric_name,
                formula = EXCLUDED.formula,
                source_table = EXCLUDED.source_table,
                description = EXCLUDED.description,
                tags = EXCLUDED.tags,
                updated_at = now()",
            params![
                &metric.slug,
                &metric.metric_name,
                &metric.formula,
                &metric.source_table,
                &metric.description,
                &metric.tags
            ]
        ).map_err(|e| format!("Failed to insert library metric {}: {}", metric.metric_name, e))?;
        
        if exists > 0 {
            updated_count += 1;
        } else {
            created_count += 1;
        }
    }
    
    eprintln!("Loaded {} library metrics from JSON ({} created, {} updated)", 
              metric_count, created_count, updated_count);
    
    Ok(())
}

/// Initialize built-in library dimensions from JSON file
fn initialize_builtin_dimensions(conn: &Connection) -> Result<(), String> {
    use duckdb::params;
    use crate::models::LibraryDimensionsJson;
    use std::path::PathBuf;
    
    // Dimensions table is already created in initialize_schema
    // Just need to load the library dimensions
    
    // Try to read library dimensions from runtime file first, fallback to embedded
    let json_content = {
        // Try runtime path (for development)
        let runtime_path = PathBuf::from("src/lib/on_init/dimensions.json");
        if runtime_path.exists() {
            eprintln!("Loading library dimensions from runtime file: {:?}", runtime_path);
            std::fs::read_to_string(&runtime_path)
                .unwrap_or_else(|_| include_str!("../../../src/lib/on_init/dimensions.json").to_string())
        } else {
            // Fallback to embedded version (for production)
            eprintln!("Loading library dimensions from embedded file");
            include_str!("../../../src/lib/on_init/dimensions.json").to_string()
        }
    };
    
    let library_dimensions: LibraryDimensionsJson = serde_json::from_str(&json_content)
        .map_err(|e| format!("Failed to parse dimensions.json: {}", e))?;
    
    let dimension_count = library_dimensions.dimensions.len();
    
    // Insert each library dimension
    let mut created_count = 0;
    let mut updated_count = 0;
    
    for dimension in library_dimensions.dimensions {
        // Check if dimension exists
        let exists: i64 = conn.query_row(
            "SELECT COUNT(*) FROM _warphead_dimensions WHERE slug = ?",
            params![&dimension.slug],
            |row| row.get(0)
        ).unwrap_or(0);
        
        // Insert or update library dimensions from JSON (allows updating when JSON changes)
        conn.execute(
            "INSERT INTO _warphead_dimensions 
             (slug, dimension_name, field_name, source_table, description, tags, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, now(), now())
             ON CONFLICT (slug) DO UPDATE SET 
                dimension_name = EXCLUDED.dimension_name,
                field_name = EXCLUDED.field_name,
                source_table = EXCLUDED.source_table,
                description = EXCLUDED.description,
                tags = EXCLUDED.tags,
                updated_at = now()",
            params![
                &dimension.slug,
                &dimension.dimension_name,
                &dimension.field_name,
                &dimension.source_table,
                &dimension.description,
                &dimension.tags
            ]
        ).map_err(|e| format!("Failed to insert library dimension {}: {}", dimension.dimension_name, e))?;
        
        if exists > 0 {
            updated_count += 1;
        } else {
            created_count += 1;
        }
    }
    
    eprintln!("Loaded {} library dimensions from JSON ({} created, {} updated)", 
              dimension_count, created_count, updated_count);
    
    Ok(())
}

/// Simple greeting command (legacy)
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to Tauri + SvelteKit!", name)
}

