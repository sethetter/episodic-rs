use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;

pub fn connect_to_db() -> diesel::ConnectionResult<PgConnection> {
    let db_url = env::var("DATABASE_URL").expect("Must configure DATABASE_URL");
    PgConnection::establish(&db_url)
}

#[derive(Serialize, Queryable)]
pub struct User {
    id: usize,
    phone: String,
    name: String,
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
