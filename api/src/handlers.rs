use actix_web::{web, Responder, HttpResponse};
use actix_web::http::StatusCode;

use crate::data;
use crate::data::users;

pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[derive(Deserialize)]
pub struct LoginRequest {
    phone: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    user: users::User,
}

pub async fn login(
    pool: web::Data<data::PgPool>,
    req: web::Json<LoginRequest>,
) -> Result<HttpResponse, HttpResponse> {
    let conn = get_conn_from_pool(pool)?;

    let phone = &req.phone;

    let user = users::find_or_insert_user(&conn, phone.to_string())
        .map_err(|_| HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR))?;

    users::new_login_token(&conn, user.id)
        .map_err(|_| HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(HttpResponse::Ok().json(LoginResponse{ user }))
}

// #[derive(Deserialize)]
// pub struct VerifyRequest {
//     user_id: i32,
//     verify_code: String,
// }

// #[post("/api/login/verify", data = "<req>")]
// pub fn login_verify(
//     req: Json<VerifyRequest>,
//     conn: DbConn,
// ) -> Result<Status, Status> {
//     let t = users::token_for_user(&conn, req.user_id)
//         .map_err(|_| Status::InternalServerError)?;

//     let token_match = t.token == req.verify_code;
//     let token_expired = t.expiry < Utc::now().naive_utc();

//     if !token_match && !token_expired {
//         return Ok(Status::BadRequest);
//     }

//     users::clear_token(&conn, t.id)
//         .map_err(|_| Status::InternalServerError)?;

//     if token_expired {
//         Err(Status::BadRequest)
//     } else {
//         Ok(Status::Ok)
//     }
// }

// #[derive(Deserialize)]
// pub struct CreateShowRequest {
//     show_id: i32,
// }

// #[derive(Serialize)]
// pub struct ListShowsResponse {
//     shows: Vec<shows::Show>,
// }

// #[post("/api/shows", data = "<req>")]
// pub fn create_show(
//     req: Json<CreateShowRequest>,
//     conn: DbConn,
// ) -> Result<Json<ListShowsResponse>, Status> {
//     let new_show = req.into_inner();

//     // get user_id
//     // get show_name from TMDB

//     shows::create_show(&conn, user_id, new_show.show_id, show_name)
//         .map_err(|_| Status::InternalServerError)?;
// }

fn get_conn_from_pool(pool: web::Data<data::PgPool>) -> Result<data::DbConn, HttpResponse> {
  pool.get().map_err(|_| HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR))
}