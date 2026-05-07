use serde::Serialize;

/// Type information for a column
#[derive(Serialize, Clone, Debug)]
pub struct ColumnTypeInfo {
    pub column_name: String,
    pub detected_type: String,      // DuckDB type (VARCHAR, INTEGER, DOUBLE, TIMESTAMP, etc.)
    pub nullable: bool,
    pub sample_values: Vec<String>,
    pub distinct_count: usize,
    pub null_count: usize,
    pub confidence: f64,            // 0.0 to 1.0
}

/// File metadata structure
#[derive(Serialize)]
pub struct FileMetadata {
    pub filename: String,
    pub folder: String,
    pub file_path: String,
    pub file_type: String,
    pub size_bytes: u64,
    pub uploaded_at: u64, // Unix timestamp in seconds
    pub row_count: Option<i64>,
    pub source_type: String, // "file", "remote", "database", or "created"
    pub source_format: String, // "csv", "json", "parquet", "mysql", "postgres", "tsv", etc.
}

/// Relationship info structure
#[derive(Serialize)]
pub struct RelationshipInfo {
    pub from_table: String,
    pub from_column: String,
    pub to_table: String,
    pub to_column: String,
}

/// Structure for library UDF definitions loaded from JSON
#[derive(serde::Deserialize)]
pub struct LibraryFunction {
    pub function_name: String,
    pub parameters: String,
    pub return_type: String,
    pub function_body: String,
    pub description: String,
    #[allow(dead_code)]
    pub category: Option<String>,
    #[allow(dead_code)]
    pub examples: Option<Vec<String>>,
}

#[derive(serde::Deserialize)]
pub struct LibraryFunctionsJson {
    pub functions: Vec<LibraryFunction>,
}

/// Structure for library chart template definitions loaded from JSON
#[derive(serde::Deserialize)]
pub struct LibraryChart {
    pub chart_name: String,
    pub chart_type: String,
    pub description: String,
    pub tags: String,
    pub chart_code: String,
    #[allow(dead_code)]
    pub config_schema: Option<String>,
    #[allow(dead_code)]
    pub metrics: Option<String>,
    #[allow(dead_code)]
    pub dimensions: Option<String>,
    #[allow(dead_code)]
    pub sample_data: Option<String>,
    #[allow(dead_code)]
    pub min_metrics: Option<i32>,
    #[allow(dead_code)]
    pub max_metrics: Option<i32>,
    #[allow(dead_code)]
    pub min_dimensions: Option<i32>,
    #[allow(dead_code)]
    pub max_dimensions: Option<i32>,
}

#[derive(serde::Deserialize)]
pub struct LibraryChartsJson {
    pub charts: Vec<LibraryChart>,
}

/// Structure for library component template definitions loaded from JSON
#[derive(serde::Deserialize)]
pub struct LibraryComponent {
    pub component_name: String,
    pub component_type: String,
    pub description: String,
    pub tags: String,
    pub html_code: String,
    #[allow(dead_code)]
    pub css_code: Option<String>,
    #[allow(dead_code)]
    pub js_code: Option<String>,
    #[allow(dead_code)]
    pub frameworks: Option<String>,
    #[allow(dead_code)]
    pub config_schema: Option<String>,
    #[allow(dead_code)]
    pub metrics: Option<String>,
    #[allow(dead_code)]
    pub dimensions: Option<String>,
    #[allow(dead_code)]
    pub sample_data: Option<String>,
    #[allow(dead_code)]
    pub min_metrics: Option<i32>,
    #[allow(dead_code)]
    pub max_metrics: Option<i32>,
    #[allow(dead_code)]
    pub min_dimensions: Option<i32>,
    #[allow(dead_code)]
    pub max_dimensions: Option<i32>,
}

#[derive(serde::Deserialize)]
pub struct LibraryComponentsJson {
    pub components: Vec<LibraryComponent>,
}

/// Structure for library metric definitions loaded from JSON
#[derive(serde::Deserialize)]
pub struct LibraryMetric {
    pub slug: String,
    pub metric_name: String,
    pub formula: String,
    pub source_table: String,
    pub description: String,
    pub tags: String,
    #[allow(dead_code)]
    pub notes: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct LibraryMetricsJson {
    pub metrics: Vec<LibraryMetric>,
}

/// Structure for library dimension definitions loaded from JSON
#[derive(serde::Deserialize)]
pub struct LibraryDimension {
    pub slug: String,
    pub dimension_name: String,
    pub field_name: String,
    pub source_table: String,
    pub description: String,
    pub tags: String,
    #[allow(dead_code)]
    pub notes: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct LibraryDimensionsJson {
    pub dimensions: Vec<LibraryDimension>,
}

/// Attached table info structure
#[derive(Serialize)]
pub struct AttachedTableInfo {
    pub table_id: String,
    pub connection_id: String,
    pub schema_name: String,
    pub table_name: String,
    pub full_name: String,
    pub access_mode: String,
    pub local_table_name: Option<String>,
    pub cached_at: Option<String>,
    pub cache_row_count: Option<i64>,
}

