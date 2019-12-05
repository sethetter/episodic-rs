use rocket::Route;
use rocket_contrib::json::Json;

use super::data;

pub fn all() -> Vec<Route>{
    routes![index, watch_list]
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/watch_list", format = "json")]
fn watch_list() -> Json<Vec<data::WatchListItem>> {
    Json(vec![])
}


