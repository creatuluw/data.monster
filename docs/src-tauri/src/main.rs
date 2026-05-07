// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Module declarations
mod state;
mod models;
mod utils;
mod commands;

// Import state and commands
use state::DuckDbState;
use commands::{
    database::*, tables::*, queries::*, files::*, folders::*, remote::*,
    metrics::*, dimensions::*, udfs::*, relationships::*, postgres::*,
    screenshots::*, routes::*, debug::*, chart_templates::*, component_templates::*,
    ingestion::*, saved_queries::*,
};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_screenshots::init())
        .manage(DuckDbState::new())
        .invoke_handler(tauri::generate_handler![
            // Database
            greet,
            initialize_duckdb,
            
            // Tables
            create_table_from_query,
            list_tables,
            get_saved_tables,
            get_table_creation_query,
            get_table_size,
            drop_table,
            
            // Queries
            execute_query,
            cancel_query,
            
            // Files
            load_csv_file,
            load_parquet_file,
            load_json_file,
            get_file_columns,
            get_file_metadata,
            get_all_files_metadata,
            get_recent_files,
            load_file_by_name,
            reload_all_files,
            delete_file,
            rename_file,
            
            // Folders
            initialize_data_folders,
            list_folders,
            create_folder,
            save_file_to_folder,
            get_file_path_in_folder,
            list_files_in_folder,
            
            // Remote
            fetch_remote_file,
            
            // UDFs
            create_udf,
            list_udfs,
            delete_udf,
            test_udf,
            reload_udfs,
            infer_column_types,
            infer_file_types,
            
            // Metrics
            create_metric,
            list_metrics,
            delete_metric,
            test_metric,
            execute_metric_with_dimensions,
            
            // Dimensions
            create_dimension,
            list_dimensions,
            delete_dimension,
            
            // Relationships
            detect_table_relationships,
            scan_all_relationships,
            
            // PostgreSQL
            create_postgres_connection,
            list_postgres_connections,
            test_postgres_connection,
            delete_postgres_connection,
            attach_postgres_database,
            detach_postgres_database,
            browse_postgres_schemas,
            browse_postgres_tables,
            import_postgres_table,
            get_attached_tables,
            
            // Screenshots
            create_screenshots_folder,
            save_screenshot_file,
            open_folder_in_explorer,
            
            // Routes
            get_all_routes,
            
            // Chart Templates
            save_chart_template,
            list_chart_templates,
            get_chart_template,
            delete_chart_template,
            
            // Component Templates
            save_component_template,
            list_component_templates,
            get_component_template,
            delete_component_template,
            
            // Saved Queries
            list_saved_queries,
            save_query,
            update_saved_query,
            delete_saved_query,
            get_saved_query,
            
            // Ingestion
            get_tables_ingestion_info,
            set_ingestion_strategy,
            ingest_table,
            register_source_for_ingestion,
            load_on_app_load_tables,
            ensure_table_ingested,
            
            // Debug
            debug_list_attached_tables,
            debug_fix_null_timestamps,
            debug_delete_table,
        ])
        .setup(|app| {
            // Initialize data folders on app startup
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = initialize_data_folders(app_handle) {
                    eprintln!("Failed to initialize data folders: {}", e);
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
