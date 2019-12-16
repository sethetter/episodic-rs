// use super::data;
use actix_web::{get, HttpResponse, Error as AWError};

#[get("/")]
pub async fn index() -> Result<HttpResponse, AWError> {
    Ok(HttpResponse::Ok().json(format!("Hello!")))
}
