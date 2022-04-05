table! {
    use diesel::sql_types::*;

    rustaceans (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        created_at -> Text,
    }
}

#[derive(Queryable, /*AsChangeset,*/ serde::Serialize, serde::Deserialize)]
pub struct Rustacean {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing)]
    pub created_at: String,
}

#[derive(Insertable, serde::Deserialize)]
#[table_name = "rustaceans"]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}
