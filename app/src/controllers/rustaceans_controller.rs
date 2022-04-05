use crate::storage::DBConnection;
use crate::storage::rustaceans_schema::{NewRustacean, Rustacean};
use crate::storage::rustaceans_model::RustaceansModel;

use rocket_contrib::json::JsonValue;

pub struct RustaceansController;

impl RustaceansController {
    pub async fn get_all_rustaceans(connection: DBConnection) -> JsonValue {
        let all_rustaceans: Vec<Rustacean> = 
            connection
                .run(move |conn| {
                    RustaceansModel::load_all(conn).expect("Error loading rustaceans")
                }).await;
        
        json!(all_rustaceans)
    }

    pub async fn get_rustacean(connection: DBConnection, id: i32) -> JsonValue {
        let rustacean: Rustacean = 
            connection
                .run(move |conn| {
                    RustaceansModel::find(conn, id).expect("Error loading rustacean")
                }).await;

        json!(rustacean)
    }

    pub async fn insert_new_rustacean(connection: DBConnection, rustacean: NewRustacean) -> JsonValue {
        let rustacean = 
            connection
                .run(move |conn| {
                    RustaceansModel::create(conn, rustacean).expect("Error creating new rustacean")
                }).await;

        json!(rustacean)
    }

    pub async fn update_rustacean(connection: DBConnection, rustacean_data: Rustacean) -> JsonValue {
        let rustacean = 
        connection
            .run(|conn| {
                RustaceansModel::update(conn, rustacean_data).expect("Error updating rustacean")
            }).await;

        json!(rustacean)
    }

    pub async fn delete_rustacean(connection: DBConnection, id: i32) {
        connection
            .run(move |conn| {
                RustaceansModel::delete(conn, id).expect("Error deleting rustacean")
            })
            .await;

        return;
    }
}