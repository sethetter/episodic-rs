#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

use actix_web::{App, HttpServer};

pub mod routes;
pub mod data;
pub mod schema;
pub mod models;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let addr = std::env::var("ADDRESS").expect("Must configure ADDRESS.");
    let port = std::env::var("PORT").expect("Must configure PORT.");

    HttpServer::new(move || {
        App::new()
            .data(data::get_db_pool())
            .service(routes::health)
            .service(routes::login)
            .service(routes::login_verify)
    })
    .bind(format!("{}:{}", addr, port))?
    .start()
    .await
}
