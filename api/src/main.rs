#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

use rocket_contrib::databases;

pub mod routes;
pub mod schema;
pub mod models;

#[database("postgres")]
pub struct DbConn(databases::diesel::PgConnection);

fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/", routes![
               routes::health,
               routes::login,
               routes::login_verify,
        ])
        .launch();
}
