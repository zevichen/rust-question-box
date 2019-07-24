use chrono::Local;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::NO_PARAMS;

type SqlitePool = Pool<SqliteConnectionManager>;

fn init() -> SqlitePool {
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = SqliteConnectionManager::file(connspec);
    let pool = r2d2::Pool::builder()
        .min_idle(Option::Some(10))
        .build(manager).expect("Failed to create pool.");
    pool
}

#[derive(Debug, Default)]
struct UserInfo {
    pub id: i64,
    pub uuid: String,
    pub nick_name: String,
}

fn main() {
//    env_logger::init();
    dotenv::dotenv().ok();

    let pool = init();
    let conn = pool.get().unwrap();

    let start_time = Local::now().timestamp_millis();

    // 无数据的情况下function都不会执行
//    for i in 0..10000 {
    let _: UserInfo = conn.query_row("select id,uuid,nick_name from user where id = 3", NO_PARAMS, |row| {
//            println!("columns = {:?}", row.columns().is_empty());
//            println!("column_count = {}", row.column_count() > 0);
        /*let mut user_info: UserInfo = Default::default();
        if !row.columns().is_empty() {
            user_info.id = row.get_unwrap(0);
            user_info.uuid = row.get_unwrap(1);
            user_info.nick_name = row.get_unwrap(2);
        }
        Ok(user_info)*/
        let uid: String = row.get_unwrap(1);
        let mut count = 1;
        if uid.is_empty() {
            count += 1;
        } else {
            count -= 1;
        }
        println!("count = {}", count);


        Ok(
            UserInfo {
                id: row.get_unwrap(0),
                uuid: row.get_unwrap(1),
                nick_name: row.get_unwrap(2),
            }
        )
    }).unwrap_or_default();
//    }
//    println!("uuid = {:?}", uuid);
    println!("spend times = {}ms", (Local::now().timestamp_millis() - start_time));

    println!("------------------------------------------")
}