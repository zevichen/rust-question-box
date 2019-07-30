use r2d2_sqlite::SqliteConnectionManager;

pub const PAGE_SIZE: i32 = 10;

pub const COMMON_DATA_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub const SOURCE_WECHAT: &str = "WECHAT";
pub const SOURCE_MOBILE: &str = "MOBILE";
pub const SOURCE_ALI: &str = "ALI";

pub type SqlitePool = r2d2::Pool<SqliteConnectionManager>;

