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
pub async fn login(req: web::Json<LoginRequest>, pool: web::Data<DbPool>) -> Result<HttpResponse, AWError> {
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

