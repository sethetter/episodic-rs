use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::result::{Error as QueryError, QueryResult};

use super::schema::{users};
use super::schema::users::dsl::*;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn get_db_pool() -> Pool {
    let db_url = std::env::var("DATABASE_URL").expect("Database URL not configured.");
    let db_manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::builder().build(db_manager).expect("Failed to create pool.")
}

#[derive(Serialize, Queryable)]
pub struct User {
    id: i32,
    phone: String,
    name: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    phone: &'a str,
}

pub fn find_or_insert_user(conn: &PgConnection, phone_input: &str) -> QueryResult<User> {
    match users.filter(phone.eq(phone_input)).first::<User>(conn) {
        // User exists? Return it.
        Ok(u) => Ok(u),

        // Not found? Create it.
        Err(QueryError::NotFound) => {
            diesel::insert_into(users::table)
                .values(NewUser{phone: phone_input})
                .get_result::<User>(conn)
        },

        // Other failure? Return the error.
        Err(err) => Err(err),
    }
}

#[derive(Serialize, Queryable)]
pub struct LoginToken {
    id: usize,
    user_id: usize,
    token: String,
}

#[derive(Serialize)]
pub struct WatchListItem {
    name: String,
    air_date: String,
    season: Option<u8>,
    number: Option<u16>,
}

#[derive(Serialize)]
pub struct Show {
    name: String,
    id: usize,
}
