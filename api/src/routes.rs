// use super::data;
use actix_web::{get, post, HttpResponse, Error as AWError};

#[get("/health")]
pub async fn health() -> Result<HttpResponse, AWError> {
    Ok(HttpResponse::Ok().json(format!("OK")))
}

#[post("/login")]
pub async fn login() -> Result<HttpResponse, AWError> {
    // find or insert user
    // return user data as response
    Ok(HttpResponse::Ok().json("Look, a user!"))
}
