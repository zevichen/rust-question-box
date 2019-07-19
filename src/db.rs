//type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;


use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

pub fn init() -> Pool<SqliteConnectionManager> {
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = SqliteConnectionManager::file(connspec);
    let pool = r2d2::Pool::builder()
        .min_idle(Option::Some(10))
        .build(manager).expect("Failed to create pool.");
    pool
}