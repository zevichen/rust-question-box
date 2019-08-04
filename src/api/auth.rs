use std::error::Error;

use actix_http::error::BlockingError;
use actix_web::{Error as AWError, HttpResponse, web};
use chrono::Local;
use futures::Future;
use futures::future::ok;
use r2d2_sqlite::SqliteConnectionManager;
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde_json::Value;

use crate::model::content::{ApiRequest, ApiResponse};
use crate::model::token::Claims;
use crate::share::{code, common};
use crate::utils::tool::jwt_encode;

const SEVEN_DAYS: usize = 7 * 24 * 60 * 60;

type SqlitePool = r2d2::Pool<SqliteConnectionManager>;

/// 小程序登录
pub fn code_session(
    item: web::Json<ApiRequest>,
    pool: web::Data<SqlitePool>,
) -> impl Future<Item=HttpResponse, Error=AWError> {
    web::block(move || {
        let mut url = "https://api.weixin.qq.com/sns/jscode2session".to_string();
        url.push_str("?appid=wx38a0c021af15f58e");
        url.push_str("&secret=a0a862fd9d65ca85da7be58d4eee0eab");
        url.push_str(format!("&js_code={}", item.code).as_str());
        url.push_str("&grant_type=authorization_code");

        let mut response = match reqwest::get(&url) {
            Ok(r) => r,
            Err(e) => return Err(ApiResponse::fail(e.to_string(), "")),
        };

        let data: Value = response.json().unwrap();
        println!("{:?}", data);

        let option_open_id = data.get("openid");
        if option_open_id.is_none() || option_open_id.unwrap().as_str().is_none() {
            let errcode = data.get("errcode").unwrap().as_i64().unwrap();
            let errmsg = data.get("errmsg").unwrap().as_str().unwrap();

            warn!("MiniAPP code_session errcode={}, errmsg={}", errcode, errmsg);
            return Err(ApiResponse::fail(errmsg.to_owned(), ""));
        };

        let session_key = data.get("session_key").unwrap().as_str().unwrap();
        //这里先用openid代替unionid
        let union_id = data.get("openid").unwrap().as_str().unwrap();
        if session_key.is_empty() || union_id.is_empty() {
            return Err(ApiResponse::fail("sessionKey or openid is empty".to_owned(), ""));
        };

        let rng = rand::thread_rng();
        let nick_name = rng.sample_iter(&Alphanumeric).take(10).collect::<String>();
        let uuid = rng.sample_iter(&Alphanumeric).take(32).collect::<String>();

        let now = Local::now().format(common::COMMON_DATA_FORMAT).to_string();

        let conn = pool.get().unwrap();

        let now_sec = (Local::now().timestamp_millis() / 1000) as usize;
        let mut clamis = Claims::default();
        clamis.union_id = union_id.to_owned();
        clamis.exp = now_sec + SEVEN_DAYS;
        clamis.iat = now_sec;
        clamis.session_key = session_key.to_owned();

        conn.query_row("select uuid,nick_name from user where union_id = $1 and is_delete = 0", &[&union_id], |row| {
            clamis.sub = row.get_unwrap(0);
            clamis.nick_name = row.get_unwrap(1);
            Ok(())
        }).ok();

        if clamis.sub.is_empty() {
            let iok = conn.execute(
                "insert or replace into user (uuid,nick_name,union_id,gmt_create,gmt_modified,source,is_delete) \
            values ($1,$2,$3,$4,$5,$6,0)",
                &[&uuid, &nick_name, union_id, &now, &now, common::SOURCE_WECHAT],
            );
            if iok.is_err() {
                return Err(ApiResponse::fail(iok.err().unwrap().description().to_owned(), ""));
            }

            clamis.sub = uuid;
            clamis.nick_name = nick_name;
        };

        let option_token = jwt_encode(clamis);
        if option_token.is_err() {
            Err(ApiResponse::fail(option_token.err().unwrap().description().to_owned(), ""))
        } else {
            Ok(ApiResponse::success(option_token.unwrap()))
        }
    }).then(|res| match res {
        Ok(r) => ok(HttpResponse::Ok().json(r)),
        Err(e) => match e {
            BlockingError::Error(e) => ok(HttpResponse::Ok().json(e)),
            BlockingError::Canceled => ok(HttpResponse::Ok()
                .json(ApiResponse::fail("system error".to_owned(), "")))
        },
    })
}

