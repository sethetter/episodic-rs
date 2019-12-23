use diesel::prelude::*;
use diesel::result::{Error as QueryError, QueryResult};

use rand::Rng;

use crate::schema::users;
use crate::schema::users::dsl::*;

use crate::schema::login_tokens;
use crate::schema::login_tokens::dsl::*;

#[derive(Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub phone: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    name: &'a str,
    phone: &'a str,
}

pub fn find_or_insert_user(
    conn: &PgConnection,
    phone_input: &str,
) -> QueryResult<User> {
    match users.filter(phone.eq(phone_input)).first::<User>(conn) {
        // User exists? Return it.
        Ok(u) => Ok(u),

        // Not found? Create it.
        Err(QueryError::NotFound) => diesel::insert_into(users::table)
            .values(NewUser {
                name: "New User",
                phone: phone_input,
            })
            .get_result::<User>(conn),

        // Other failure? Return the error.
        Err(err) => Err(err),
    }
}

#[derive(Serialize, Queryable)]
pub struct LoginToken {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
}

#[derive(Insertable)]
#[table_name = "login_tokens"]
pub struct NewLoginToken {
    user_id: i32,
    token: String,
}

pub fn new_login_token(
    conn: &PgConnection,
    for_user_id: i32
) -> QueryResult<usize> {
    let new_token = new_token();

    diesel::insert_into(login_tokens::table)
        .values(NewLoginToken {
            user_id: for_user_id,
            token: new_token.clone(),
        })
        .on_conflict(user_id)
        .do_update()
        .set(token.eq(new_token))
        .returning(login_tokens::id)
        .execute(conn)
}

fn new_token() -> String {
    let mut rng = rand::thread_rng();
    (0..6).fold("".to_string(), |out, _| {
        format!("{}{}", out, rng.gen_range(0, 9).to_string())
    })
}

pub fn token_for_user(
    conn: &PgConnection,
    for_user_id: i32
) -> QueryResult<LoginToken> {
    // TODO: allow tokens to expire
    login_tokens
        .filter(login_tokens::user_id.eq(for_user_id))
        .first::<LoginToken>(conn)
}

pub fn clear_token(
    conn: &PgConnection,
    token_id: i32,
) -> QueryResult<usize> {
    diesel::delete(login_tokens.filter(login_tokens::id.eq(token_id))).execute(conn)
}
