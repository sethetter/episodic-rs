#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;

use actix_web::{web, App};
use std::env;

pub mod handlers;
pub mod data;
pub mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let db_conn_str = env::var("DATABASE_URL").expect("Must provide DATABASE_URL");
    let host = env::var("APP_HOST").expect("Must provide APP_HOST");
    let port = env::var("APP_PORT").expect("Must provide APP_PORT");

    let running_at = format!("{}:{}", host, port);
    println!("Running at {}", running_at);

    actix_web::HttpServer::new(move || {
        App::new()
            .data(data::init_pool(&db_conn_str))
            .route("/api/health", web::get().to(handlers::health))
            .route("/api/login", web::post().to(handlers::login))
    })
    .bind(running_at)?
    .run()
    .await
}

// fn with_db(pool: PgPool) -> impl Filter<Extract = (DbConn,), Error = PoolError> {
//     warp::any().map(|| data::get_conn_from_pool(pool))
// }