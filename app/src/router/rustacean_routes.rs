use rocket::response::status;
use rocket_contrib::json::{Json, JsonValue};

use crate::auth::basic_auth::BasicAuth;
use crate::controllers::rustaceans_controller::RustaceansController;
use crate::storage::rustaceans_schema::*;
use crate::storage::DBConnection;

#[get("/rustaceans")]
pub async fn get_rustaceans(_auth: BasicAuth, connection: DBConnection) -> JsonValue {
    RustaceansController::get_all_rustaceans(connection).await
}

#[get("/rustaceans/<id>")]
pub async fn get_rustaceans_id(_auth: BasicAuth, connection: DBConnection, id: i32) -> JsonValue {
    RustaceansController::get_rustacean(connection, id).await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
pub async fn post_rustaceans(
    _auth: BasicAuth,
    connection: DBConnection,
    new_rustacean: Json<NewRustacean>,
) -> JsonValue {
    RustaceansController::insert_new_rustacean(connection, new_rustacean.into_inner()).await
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
pub async fn put_rustaceans_id(
    _auth: BasicAuth,
    connection: DBConnection,
    id: i32,
    rustacean: Json<Rustacean>,
) -> JsonValue {
    let mut rustacean_to_save = rustacean.into_inner();
    rustacean_to_save.id = id;

    RustaceansController::update_rustacean(connection, rustacean_to_save).await
}

#[delete("/rustaceans/<id>")]
pub async fn delete_rustaceans_id(
    _auth: BasicAuth,
    connection: DBConnection,
    id: i32,
) -> status::NoContent {
    RustaceansController::delete_rustacean(connection, id).await;
    status::NoContent
}
