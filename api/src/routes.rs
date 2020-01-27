use {
    rocket::{
        http::Status,
    },
    rocket_contrib::{
        json::Json,
    },
    chrono::Utc,
    crate::{
        DbConn,
        models::users,
    },
};

#[get("/api/health")]
pub fn health() -> &'static str {
    "OK"
}

#[derive(Deserialize)]
pub struct LoginRequest {
    phone: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    user: users::User,
}

#[post("/api/login", format = "application/json", data = "<req>")]
pub fn login(
    req: Json<LoginRequest>,
    conn: DbConn,
) -> Result<Json<LoginResponse>, Status> {
    let phone = req.into_inner().phone;

    let user = users::find_or_insert_user(&conn, phone.as_str())
        .map_err(|_| { Status::InternalServerError })?;

    users::new_login_token(&conn, user.id)
        .map_err(|_| { Status::InternalServerError })?;

    Ok(Json(LoginResponse{ user }))
}

#[derive(Deserialize)]
pub struct VerifyRequest {
    user_id: i32,
    verify_code: String,
}

#[post("/api/login/verify", data = "<req>")]
pub fn login_verify(
    req: Json<VerifyRequest>,
    conn: DbConn,
) -> Result<Status, Status> {
    let t = users::token_for_user(&conn, req.user_id)
        .map_err(|_| Status::InternalServerError)?;

    let token_match = t.token == req.verify_code;
    let token_expired = t.expiry < Utc::now().naive_utc();

    if !token_match && !token_expired {
        return Ok(Status::BadRequest);
    }

    users::clear_token(&conn, t.id)
        .map_err(|_| Status::InternalServerError)?;

    if token_expired {
        Err(Status::BadRequest)
    } else {
        Ok(Status::Ok)
    }
}
