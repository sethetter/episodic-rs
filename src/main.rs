#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

use rocket::request::{Request, Outcome, FromRequest};
use rocket::http::Status;

pub mod routes;
pub mod data;

pub struct DbConn(diesel::PgConnection);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = diesel::ConnectionError;

    fn from_request(req: &'a Request<'r>) -> Outcome<DbConn, Self::Error> {
        match data::connect_to_db() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(err) => Outcome::Failure((Status::ServiceUnavailable, err)),
        }
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes::all())
        .launch();
}
