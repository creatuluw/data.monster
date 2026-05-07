// Commands module - organized by feature area
pub mod database;
pub mod tables;
pub mod queries;
pub mod files;
pub mod folders;
pub mod remote;
pub mod metrics;
pub mod dimensions;
pub mod udfs;
pub mod relationships;
pub mod postgres;
pub mod screenshots;
pub mod routes;
pub mod debug;
pub mod chart_templates;
pub mod component_templates;
pub mod init_data;
pub mod ingestion;
pub mod saved_queries;

// Commands are accessed directly through their modules in main.rs
// No re-exports needed here


