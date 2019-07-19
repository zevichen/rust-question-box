use actix_session::Session;
use actix_web::{Error, HttpResponse, web};
use chrono::Local;
use futures::Future;
use futures::future::ok;
use log::Level;
use r2d2_sqlite::SqliteConnectionManager;
use rand_core::RngCore;

use model::content::Content;
use model::user::{UserInfo, UuidSigninUser};
use share::common;

type SqlitePool = r2d2::Pool<SqliteConnectionManager>;

/// login
pub fn login(item: web::Json<UuidSigninUser>,
             pool: web::Data<SqlitePool>,
             session: Session) -> impl Future<Item=HttpResponse, Error=Error> {
    if log_enabled!(Level::Debug) {
        info!("Params = {:?}", &item)
    }

    let opt_user_info = session.get::<UserInfo>("user_info").unwrap();
    if !opt_user_info.is_none() {
        return ok(HttpResponse::Ok().json(Content::new(opt_user_info.unwrap().uuid)));
    }

    if item.union_id.is_empty() || item.session_key.is_empty() {
        return ok(HttpResponse::Ok().json(Content::err("", "union_id or session_key is empty.")));
    }

    let conn = pool.get().unwrap();

    let mut uuid = conn.query_row("select uuid from user where union_id = $1 limit 1", &[&item.union_id], |row| {
        row.get::<_, String>(0)
    }).expect("select uuid exception.");

    if !uuid.is_empty() {
        return ok(HttpResponse::Ok().json(Content::new(uuid)));
    }

    let now = Local::now().format(common::DATE_FORMAT_1).to_string();
    let mut rng = rand::thread_rng();
    uuid = format!("{}", rng.next_u64());

    conn.execute(
        "insert or replace into user (uuid,nick_name,union_id,gmt_create,gmt_modified,source,is_delete) \
            values ($1,$2,$3,$4,$5,$6,0)",
        &[&uuid, &item.nick_name, &item.union_id, &now, &now, common::SOURCE_WECHAT],
    ).expect("user insert or replace failed.");

    let mut user_info = UserInfo::new();
    user_info.is_login = true;
    user_info.uuid = uuid.to_owned();
    user_info.nick_name = item.nick_name.to_owned();
    user_info.token =
    session.set::<UserInfo>("user_info", user_info).expect("Failed setting user_info into session");

    return ok(HttpResponse::Ok().json(Content::new(uuid)));
}

/// user_info
pub fn info(session: Session) -> impl Future<Item=HttpResponse, Error=Error> {
    let opt_user_info = session.get::<UserInfo>("user_info").unwrap_or_default();

    let mut user_info;
    if !opt_user_info.is_none() {
        user_info = opt_user_info.unwrap();
        user_info.union_id = "";
    } else {
        user_info = UserInfo::new();
    }
    ok(HttpResponse::Ok().json(Content::new(user_info)))
}

/// logout
pub fn logout(session: Session) -> impl Future<Item=HttpResponse, Error=Error> {
    session.clear();
    ok(HttpResponse::Ok().json(Content::err("", "注销成功")))
}
