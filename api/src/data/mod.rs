pub mod users;
// pub mod shows;

use diesel::PgConnection;
use diesel::r2d2::{Pool, PooledConnection, ConnectionManager};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConn = PooledConnection<ConnectionManager<PgConnection>>;

pub fn init_pool(conn_str: &str) -> PgPool {
  let manager = ConnectionManager::<PgConnection>::new(conn_str);
  Pool::builder().build(manager).expect("Failed to creat DB connection pool")
}