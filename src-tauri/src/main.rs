#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod models;
mod state;
mod utils;

use commands::{
    database::*, field_functions::*, files::*, folders::*, internal_db::*, items::*, labels::*,
    local_llm::*, pages::*, postgres::*, queries::*, relationships::*, saved_queries::*,
    settings::*, tables::*, workspace::*,
};
use commands::local_llm::LlmState;
use state::DuckDbState;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(DuckDbState::default())
        .manage(
            LlmState::new()
                .expect("Failed to initialize LLM state"),
        )
        .invoke_handler(tauri::generate_handler![
            initialize_duckdb,
            shutdown_duckdb,
            reset_all_data,
            choose_workspace_folder,
            get_workspace_path,
            set_workspace_path,
            list_workspaces,
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
            list_field_functions,
            create_field_function,
            update_field_function,
            delete_field_function,
            list_saved_queries,
            save_query,
            update_saved_query,
            delete_saved_query,
            list_pages,
            get_page,
            save_page,
            delete_page,
            list_master_items,
            get_master_item,
            save_master_item,
            delete_master_item,
            list_relationships,
            save_relationship,
            delete_relationship,
            list_internal_tables,
            query_internal_table,
            update_internal_row,
            delete_internal_row,
            get_settings,
            save_settings,
            connect_postgres,
            list_postgres_tables,
            generate_pg_ingest_sql,
            detect_system_ram,
            list_available_models,
            download_model,
            cancel_download,
            list_downloaded_models,
            delete_model,
            load_model,
            unload_model,
            is_model_loaded,
            generate_chat,
            remote_chat,
            stop_generation,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
