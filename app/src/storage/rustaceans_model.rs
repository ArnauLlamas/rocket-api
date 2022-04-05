use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, MysqlConnection};

use crate::storage::rustaceans_schema::{rustaceans, Rustacean, NewRustacean};


pub struct RustaceansModel;

impl RustaceansModel {
    pub fn load_all(conn: &MysqlConnection) -> QueryResult<Vec<Rustacean>> {
        //rustaceans::table.limit(100).load::<Rustacean>(conn)
        rustaceans::table.load(conn)
    }

    pub fn find(conn: &MysqlConnection, id: i32) -> QueryResult<Rustacean> {
        //rustaceans::table.find(id).get_result::<Rustacean>(conn)
        rustaceans::table.find(id).first(conn)
    }

    pub fn create(conn: &MysqlConnection, new_rustacean: NewRustacean) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table)
            .values(new_rustacean)
            .execute(conn)?;

        let last_id = Self::last_id(conn)?;

        Self::find(conn, last_id)
    }

    pub fn update(conn: &MysqlConnection, rustacean: Rustacean) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table.find(rustacean.id))
            .set((
                rustaceans::name.eq(rustacean.name.to_owned()),
                rustaceans::email.eq(rustacean.email.to_owned()),
            ))
            .execute(conn)?;

        Self::find(conn, rustacean.id)
    }

    pub fn delete (conn: &MysqlConnection, rustacean_id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.find(rustacean_id))
            .execute(conn)
    }

    fn last_id(conn: &MysqlConnection) -> QueryResult<i32> {
        rustaceans::table
            .select(rustaceans::id)
            .order(rustaceans::id.desc())
            .first(conn)
    }
}
