use duckdb::Connection;
use std::path::PathBuf;
use std::fs;
use tauri::{AppHandle, Manager};
use crate::commands::files::get_data_dir;
use crate::commands::remote::fetch_remote_file;

/// Initialize sample data on first run
/// This loads the sales_data.csv and creates the star schema if not already present
pub async fn initialize_sample_data(conn: &Connection, app: &AppHandle) -> Result<String, String> {
    // Check if sales_data initialization has already been done
    let check_sales_data = "SELECT COUNT(*) FROM information_schema.tables WHERE table_name = 'sales_data'";
    let sales_data_exists: Result<i64, _> = conn.query_row(check_sales_data, [], |row| row.get(0));
    
    let sales_data_initialized = sales_data_exists.map(|count| count > 0).unwrap_or(false);
    
    // Check if sales_history has been loaded
    let check_sales_history = "SELECT COUNT(*) FROM information_schema.tables WHERE table_name = 'sales_history'";
    let sales_history_exists: Result<i64, _> = conn.query_row(check_sales_history, [], |row| row.get(0));
    let sales_history_initialized = sales_history_exists.map(|count| count > 0).unwrap_or(false);
    
    // Check if model tables exist
    let check_model_tables = "SELECT COUNT(*) FROM information_schema.tables WHERE table_name IN ('dim_customers', 'dim_products', 'dim_time')";
    let model_tables_exist: Result<i64, _> = conn.query_row(check_model_tables, [], |row| row.get(0));
    let has_model_tables = model_tables_exist.map(|count| count >= 3).unwrap_or(false);
    
    // If both source tables and model tables exist, we're fully initialized
    if sales_data_initialized && sales_history_initialized && has_model_tables {
        eprintln!("✅ All tables exist - fully initialized");
        return Ok("Sample data already initialized".to_string());
    }
    
    // If source tables exist but model tables don't, recreate the star schema
    if sales_data_initialized && sales_history_initialized && !has_model_tables {
        eprintln!("📊 Source tables exist but model tables missing. Creating star schema...");
        create_star_schema(conn, app)?;
        return Ok("Star schema created successfully".to_string());
    }
    
    // If only sales_data exists but not sales_history, we need to load remote data and create schema
    if sales_data_initialized && !sales_history_initialized {
        eprintln!("📊 sales_data exists, but sales_history missing. Loading remote data...");
        
        // Load remote sales_history
        eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        eprintln!("📡 REMOTE DATA LOADING");
        eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        load_sales_history_remote(conn, app).await?;
        eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        
        // Create star schema
        create_star_schema(conn, app)?;
        
        return Ok("Remote sales_history loaded and star schema created".to_string());
    }
    
    // If we get here, we need to do full initialization
    eprintln!("🚀 Starting full initialization...");
    
    // Get the path to the init assets
    let init_csv_path = get_init_csv_path(app)?;
    
    // Check if the CSV file exists
    if !init_csv_path.exists() {
        return Err("Init CSV file not found. Please ensure src/lib/on_init/sales_data.csv exists.".to_string());
    }
    
    // Copy the CSV file to the data/main folder for persistence
    let data_folder_path = copy_csv_to_data_folder(app, &init_csv_path)?;
    
    eprintln!("📊 Loading sample data from: {}", data_folder_path);
    
    let csv_path_str = data_folder_path.replace("\\", "\\\\");
    
    // Step 1: Load sales_data.csv as source table
    let load_csv_query = format!(
        "CREATE TABLE sales_data AS SELECT * FROM read_csv_auto('{}')",
        csv_path_str
    );
    
    conn.execute(&load_csv_query, [])
        .map_err(|e| format!("Failed to load sales_data CSV: {}", e))?;
    
    // Register as source table
    conn.execute(
        "INSERT INTO _warphead_table_metadata (table_name, table_type, created_at) 
         VALUES ('sales_data', 'source', now()) 
         ON CONFLICT (table_name) DO NOTHING",
        []
    ).map_err(|e| format!("Failed to register sales_data metadata: {}", e))?;
    
    let row_count: i64 = conn.query_row("SELECT COUNT(*) FROM sales_data", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    
    eprintln!("✅ Loaded sales_data table with {} rows", row_count);
    
    // Step 2: Load sales_history from remote URL
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("📡 REMOTE DATA LOADING");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    load_sales_history_remote(conn, app).await?;
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Step 3: Create star schema (dimensions and fact tables)
    create_star_schema(conn, app)?;
    
    Ok(format!("Sample data initialized successfully: {} rows loaded (sales_data), star schema created", row_count))
}

/// Get the path to the init CSV file
fn get_init_csv_path(app: &AppHandle) -> Result<PathBuf, String> {
    let current_dir = std::env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))?;
    
    // In development, when running from src-tauri directory, go up one level
    let dev_path_from_tauri = current_dir.parent()
        .ok_or("Failed to get parent directory")?
        .join("src")
        .join("lib")
        .join("on_init")
        .join("sales_data.csv");
    
    if dev_path_from_tauri.exists() {
        eprintln!("Found CSV at dev path: {:?}", dev_path_from_tauri);
        return Ok(dev_path_from_tauri);
    }
    
    // Try from current directory (if running from project root)
    let dev_path = PathBuf::from("src/lib/on_init/sales_data.csv");
    if dev_path.exists() {
        eprintln!("Found CSV at project root path: {:?}", dev_path);
        return Ok(dev_path);
    }
    
    // Try relative to current directory
    let relative_path = current_dir.join("src").join("lib").join("on_init").join("sales_data.csv");
    if relative_path.exists() {
        eprintln!("Found CSV at relative path: {:?}", relative_path);
        return Ok(relative_path);
    }
    
    // Try to get the resource path (works in production)
    if let Ok(resource_path) = app.path().resource_dir() {
        let csv_path = resource_path
            .join("src")
            .join("lib")
            .join("on_init")
            .join("sales_data.csv");
        
        if csv_path.exists() {
            eprintln!("Found CSV at resource path: {:?}", csv_path);
            return Ok(csv_path);
        }
        
        // Try production path
        let prod_path = resource_path
            .join("on_init")
            .join("sales_data.csv");
        
        if prod_path.exists() {
            eprintln!("Found CSV at production path: {:?}", prod_path);
            return Ok(prod_path);
        }
    }
    
    Err(format!("Init CSV file not found. Tried paths:\n  - {:?}\n  - {:?}\n  - {:?}\n  Current dir: {:?}", 
        dev_path_from_tauri, dev_path, relative_path, current_dir))
}

/// Copy the CSV file to the data/main folder for persistence and access
fn copy_csv_to_data_folder(app: &AppHandle, source_path: &PathBuf) -> Result<String, String> {
    // Get the data directory
    let data_dir = get_data_dir(app)?;
    
    // Create the main folder if it doesn't exist
    let main_folder = data_dir.join("main");
    if !main_folder.exists() {
        fs::create_dir_all(&main_folder)
            .map_err(|e| format!("Failed to create main folder: {}", e))?;
    }
    
    // Target path for the CSV file
    let target_path = main_folder.join("sales_data.csv");
    
    // Only copy if it doesn't exist
    if !target_path.exists() {
        fs::copy(source_path, &target_path)
            .map_err(|e| format!("Failed to copy CSV to data folder: {}", e))?;
        
        eprintln!("✅ Copied sales_data.csv to: {}", target_path.display());
    } else {
        eprintln!("ℹ️  sales_data.csv already exists in data folder");
    }
    
    target_path.to_str()
        .ok_or("Invalid target path encoding".to_string())
        .map(|s| s.to_string())
}

/// Load sales_history.csv from remote URL
async fn load_sales_history_remote(conn: &Connection, app: &AppHandle) -> Result<(), String> {
    eprintln!("🌐 Checking for sales_history table...");
    
    // Check if sales_history table already exists
    let check_query = "SELECT COUNT(*) FROM information_schema.tables WHERE table_name = 'sales_history'";
    let already_exists: Result<i64, _> = conn.query_row(check_query, [], |row| row.get(0));
    
    if let Ok(count) = already_exists {
        if count > 0 {
            eprintln!("ℹ️  sales_history table already exists, skipping remote load");
            return Ok(());
        }
    }
    
    // Fetch the remote file
    const SALES_HISTORY_URL: &str = "http://patrick.te9.nl/data/sales_history.csv";
    const REMOTE_FOLDER: &str = "init_data";
    
    eprintln!("");
    eprintln!("🌐 REMOTE FILE LOADING INITIATED");
    eprintln!("   Source: {}", SALES_HISTORY_URL);
    eprintln!("   Target: data/remote/{}/sales_history.csv", REMOTE_FOLDER);
    eprintln!("   Size: ~200MB (1.2M+ records)");
    eprintln!("");
    eprintln!("⬇️  Downloading... (this may take 10-30 seconds)");
    
    let saved_path = match fetch_remote_file(
        app.clone(),
        SALES_HISTORY_URL.to_string(),
        REMOTE_FOLDER.to_string()
    ).await {
        Ok(path) => {
            eprintln!("");
            eprintln!("✅ DOWNLOAD COMPLETE");
            eprintln!("   Saved to: {}", path);
            eprintln!("");
            path
        },
        Err(e) => {
            eprintln!("");
            eprintln!("⚠️  DOWNLOAD FAILED: {}", e);
            eprintln!("   The app will continue without historical data.");
            eprintln!("   You can manually add the file later via /data/remote");
            eprintln!("");
            return Ok(()); // Continue without error to not block initialization
        }
    };
    
    // Load the CSV into DuckDB
    eprintln!("📥 Loading remote file into DuckDB...");
    let csv_path_str = saved_path.replace("\\", "\\\\");
    
    let load_csv_query = format!(
        "CREATE TABLE sales_history AS SELECT * FROM read_csv_auto('{}')",
        csv_path_str
    );
    
    conn.execute(&load_csv_query, [])
        .map_err(|e| format!("Failed to load sales_history CSV: {}", e))?;
    
    // Register as source table
    conn.execute(
        "INSERT INTO _warphead_table_metadata (table_name, table_type, created_at) 
         VALUES ('sales_history', 'source', now()) 
         ON CONFLICT (table_name) DO NOTHING",
        []
    ).map_err(|e| format!("Failed to register sales_history metadata: {}", e))?;
    
    let history_count: i64 = conn.query_row("SELECT COUNT(*) FROM sales_history", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    
    eprintln!("✅ Loaded sales_history table with {} rows", history_count);
    
    Ok(())
}

/// Get the path to the create-star-schema.sql file
fn get_star_schema_sql_path(app: &AppHandle) -> Result<PathBuf, String> {
    let current_dir = std::env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))?;
    
    // In development, when running from src-tauri directory, go up one level
    let dev_path_from_tauri = current_dir.parent()
        .ok_or("Failed to get parent directory")?
        .join("src")
        .join("lib")
        .join("on_init")
        .join("create-star-schema.sql");
    
    if dev_path_from_tauri.exists() {
        eprintln!("Found SQL file at dev path: {:?}", dev_path_from_tauri);
        return Ok(dev_path_from_tauri);
    }
    
    // Try from current directory (if running from project root)
    let dev_path = PathBuf::from("src/lib/on_init/create-star-schema.sql");
    if dev_path.exists() {
        eprintln!("Found SQL file at project root path: {:?}", dev_path);
        return Ok(dev_path);
    }
    
    // Try relative to current directory
    let relative_path = current_dir.join("src").join("lib").join("on_init").join("create-star-schema.sql");
    if relative_path.exists() {
        eprintln!("Found SQL file at relative path: {:?}", relative_path);
        return Ok(relative_path);
    }
    
    // Try to get the resource path (works in production)
    if let Ok(resource_path) = app.path().resource_dir() {
        let sql_path = resource_path
            .join("src")
            .join("lib")
            .join("on_init")
            .join("create-star-schema.sql");
        
        if sql_path.exists() {
            eprintln!("Found SQL file at resource path: {:?}", sql_path);
            return Ok(sql_path);
        }
        
        // Try production path
        let prod_path = resource_path
            .join("on_init")
            .join("create-star-schema.sql");
        
        if prod_path.exists() {
            eprintln!("Found SQL file at production path: {:?}", prod_path);
            return Ok(prod_path);
        }
    }
    
    Err(format!("create-star-schema.sql file not found. Tried paths:\n  - {:?}\n  - {:?}\n  - {:?}\n  Current dir: {:?}", 
        dev_path_from_tauri, dev_path, relative_path, current_dir))
}

/// Create star schema by executing the create-star-schema.sql file
fn create_star_schema(conn: &Connection, app: &AppHandle) -> Result<(), String> {
    eprintln!("");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("🏗️  CREATING STAR SCHEMA");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("");
    
    // Get the path to the SQL file
    let sql_file_path = get_star_schema_sql_path(app)?;
    
    // Read the SQL file
    let sql_content = fs::read_to_string(&sql_file_path)
        .map_err(|e| format!("Failed to read create-star-schema.sql: {}", e))?;
    
    eprintln!("✅ Loaded SQL file from: {:?}", sql_file_path);
    eprintln!("📝 File size: {} bytes ({} KB)", sql_content.len(), sql_content.len() / 1024);
    eprintln!("");
    
    // Split the SQL file into individual statements
    // We'll split by semicolon and filter out comments and empty statements
    let statements: Vec<&str> = sql_content
        .split(';')
        .map(|s| s.trim())
        .filter(|s| {
            !s.is_empty() 
            && !s.lines().all(|line| {
                let trimmed = line.trim();
                trimmed.is_empty() || trimmed.starts_with("--")
            })
        })
        .collect();
    
    eprintln!("📊 Executing {} SQL statements...", statements.len());
    eprintln!("");
    
    let mut success_count = 0;
    let mut skip_count = 0;
    let mut error_count = 0;
    
    for (i, statement) in statements.iter().enumerate() {
        // Skip if statement is just comments
        if statement.lines().all(|line| line.trim().starts_with("--") || line.trim().is_empty()) {
            continue;
        }
        
        // Get a preview of what we're executing (first meaningful line)
        let preview = statement.lines()
            .find(|line| {
                let trimmed = line.trim();
                !trimmed.starts_with("--") && !trimmed.is_empty()
            })
            .unwrap_or("...")
            .trim();
        
        let preview_short = if preview.len() > 60 {
            format!("{}...", &preview[..60])
        } else {
            preview.to_string()
        };
        
        eprint!("  [{}/{}] {}... ", i + 1, statements.len(), preview_short);
        
        // Execute the statement
        match conn.execute(statement, []) {
            Ok(_) => {
                eprintln!("✅");
                success_count += 1;
            }
            Err(e) => {
                // Check if it's an "already exists" error - these are OK
                let error_msg = e.to_string();
                if error_msg.contains("already exists") || error_msg.contains("duplicate key") {
                    eprintln!("⚠️  (already exists)");
                    skip_count += 1;
                } else {
                    eprintln!("❌");
                    eprintln!("     ERROR: {}", e);
                    eprintln!("     Statement preview: {}", preview_short);
                    error_count += 1;
                    
                    // Continue with other statements to see all errors
                    // but track that we had failures
                }
            }
        }
    }
    
    eprintln!("");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("📊 Star Schema Creation Summary");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("   ✅ Successful: {}", success_count);
    eprintln!("   ⚠️  Skipped: {}", skip_count);
    eprintln!("   ❌ Errors: {}", error_count);
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("");
    
    if error_count > 0 {
        return Err(format!("Star schema creation completed with {} errors. Check the logs above for details.", error_count));
    }
    
    // Verify that the model tables were actually created
    let check_dims = "SELECT COUNT(*) FROM information_schema.tables WHERE table_name IN ('dim_customers', 'dim_products', 'dim_time')";
    let dims_count: Result<i64, _> = conn.query_row(check_dims, [], |row| row.get(0));
    
    let check_facts = "SELECT COUNT(*) FROM information_schema.tables WHERE table_name IN ('fact_sales', 'fact_sales_history')";
    let facts_count: Result<i64, _> = conn.query_row(check_facts, [], |row| row.get(0));
    
    let dims_created = dims_count.unwrap_or(0);
    let facts_created = facts_count.unwrap_or(0);
    
    eprintln!("✅ Verification: {} dimension tables, {} fact tables created", dims_created, facts_created);
    eprintln!("");
    
    if dims_created < 3 || facts_created < 2 {
        return Err(format!("Star schema verification failed: Expected 3 dimension tables and 2 fact tables, but found {} dimensions and {} facts", dims_created, facts_created));
    }
    
    eprintln!("✅ Star schema created and verified successfully");
    eprintln!("");
    
    Ok(())
}

