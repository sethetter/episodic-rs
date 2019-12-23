use diesel::prelude::{PgConnection};
use diesel::r2d2::{self, ConnectionManager};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn get_db_pool() -> DbPool {
    let db_url = std::env::var("DATABASE_URL").expect("Database URL not configured.");
    let db_manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::builder().build(db_manager).expect("Failed to create pool.")
}

// #[derive(Serialize)]
// pub struct WatchListItem {
//     name: String,
//     air_date: String,
//     season: Option<u8>,
//     number: Option<u16>,
// }

// #[derive(Serialize)]
// pub struct Show {
//     name: String,
//     id: usize,
// }
