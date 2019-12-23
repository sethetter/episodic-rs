use {
    actix_web::{
        web, get, post, HttpResponse, Error as AWError
    },
    super::data::DbPool,
    crate::models::users,
    serde::Deserialize,
};

#[get("/health")]
pub async fn health() -> Result<HttpResponse, AWError> {
    Ok(HttpResponse::Ok().json(format!("OK")))
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    phone: String,
}

#[post("/login")]
pub async fn login(
    req: web::Json<LoginRequest>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, AWError> {
    let phone = req.into_inner().phone;
    let conn = &pool.get().unwrap();

    match users::find_or_insert_user(conn.clone(), phone.as_str()) {
        Ok(user) => match users::new_login_token(conn.clone(), user.id) {
            Ok(_) => Ok(HttpResponse::Ok().json(user)),
            Err(e) => Ok(HttpResponse::InternalServerError().body(format!("{:?}", e))),
        },
        Err(e) => Ok(HttpResponse::InternalServerError().body(format!("{:?}", e))),
    }
}

#[derive(Serialize, Deserialize)]
pub struct VerifyRequest {
    user_id: i32,
    verify_code: String,
}

#[post("/login/verify")]
pub async fn login_verify(
    raw_req: web::Json<VerifyRequest>,
    pool: web::Data<DbPool>
) -> Result<HttpResponse, AWError> {
    let conn = &pool.get().unwrap();
    let req = raw_req.into_inner();

    match users::token_for_user(conn.clone(), req.user_id) {
        Ok(t) => {
            if t.token == req.verify_code {
                match users::clear_token(conn.clone(), t.id) {
                    Ok(_) => Ok(HttpResponse::Ok().json("Success!")),
                    Err(e) => Ok(HttpResponse::InternalServerError().body(format!("{:?}", e))),
                }
            } else {
                Ok(HttpResponse::BadRequest().json("Invalid"))
            }
        },
        Err(e) => Ok(HttpResponse::InternalServerError().body(format!("{:?}", e))),
    }
}
