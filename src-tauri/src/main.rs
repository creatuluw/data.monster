#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod models;
mod state;
mod utils;

use commands::{
    database::*, files::*, folders::*, internal_db::*, labels::*, postgres::*, queries::*,
    saved_queries::*, settings::*, tables::*, workspace::*,
};
use state::DuckDbState;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(DuckDbState::default())
        .invoke_handler(tauri::generate_handler![
            initialize_duckdb,
            shutdown_duckdb,
            choose_workspace_folder,
            get_workspace_path,
            set_workspace_path,
            load_csv_file,
            load_parquet_file,
            load_json_file,
            get_file_columns,
            get_file_size,
            download_url_to_workspace,
            preview_file,
            initialize_data_folders,
            list_folders,
            create_folder,
            execute_query,
            cancel_query,
            list_tables,
            drop_table,
            create_table_from_query,
            rename_table,
            save_table_source,
            get_table_source,
            get_table_types,
            refresh_table_from_source,
            save_table_labels,
            get_table_labels,
            get_all_tags,
            get_all_groups,
            list_saved_queries,
            save_query,
            update_saved_query,
            delete_saved_query,
            list_internal_tables,
            query_internal_table,
            update_internal_row,
            delete_internal_row,
            get_settings,
            save_settings,
            connect_postgres,
            list_postgres_tables,
            generate_pg_ingest_sql,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
