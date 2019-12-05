#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

pub mod routes;
pub mod data;

fn main() {
    rocket::ignite()
        .mount("/", routes::all())
        .launch();
}
