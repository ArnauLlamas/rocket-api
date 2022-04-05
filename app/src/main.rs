#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

mod auth;

mod router;
use router::catchers;
use router::rustacean_routes;

mod controllers;
mod storage;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                rustacean_routes::get_rustaceans,
                rustacean_routes::get_rustaceans_id,
                rustacean_routes::post_rustaceans,
                rustacean_routes::put_rustaceans_id,
                rustacean_routes::delete_rustaceans_id
            ],
        )
        .register(
            "/",
            catchers![
                catchers::unauthorized,
                catchers::not_found,
                catchers::unprocessable_entity,
                catchers::internal_error,
            ],
        )
        .attach(storage::DBConnection::fairing())
        .launch()
        .await;
}
