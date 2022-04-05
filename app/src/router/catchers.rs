use rocket_contrib::json::JsonValue;

#[catch(404)]
pub fn not_found() -> JsonValue {
    json!("Not found")
}

#[catch(401)]
pub fn unauthorized() -> JsonValue {
    json!("Unauthorized")
}

#[catch(422)]
pub fn unprocessable_entity() -> JsonValue {
    json!("Unprocessable Entity")
}

#[catch(500)]
pub fn internal_error() -> JsonValue {
    json!("Internal Server Error")
}
