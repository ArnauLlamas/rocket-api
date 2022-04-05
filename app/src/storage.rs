pub mod rustaceans_model;
pub mod rustaceans_schema;

#[database("mysql")]
pub struct DBConnection(diesel::MysqlConnection);