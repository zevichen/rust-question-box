use std::error::Error;

use actix::fut::err;
use actix_http::Error;
use actix_web::{HttpRequest, HttpResponse};
use chrono::Local;
use futures::Future;
use futures::future::ok;
use hyper::server::conn::Http;
use jwt::{Algorithm, Header, TokenData, Validation};
use jwt::errors::ErrorKind;
use serde_json::Value;
use serde_json::value::Value::Number;

use crate::model::content::ApiResponse;
use crate::model::token::Claims;

const SEVEN_DAYS: usize = 7 * 24 * 60 * 60 * 1000;

/// 小程序获取session
pub fn code_session(code: &str, req: HttpRequest) -> impl Future<Item=HttpResponse, Error=Error> {
    let mut url = "https://api.weixin.qq.com/sns/jscode2session".to_string();
    url.push_str("?appid=wx38a0c021af15f58e");
    url.push_str("&secret=a0a862fd9d65ca85da7be58d4eee0eab");
    url.push_str(format!("&js_code={}", code).as_str());
    url.push_str("&grant_type=authorization_code");

    let mut response = match reqwest::get(&url) {
        Ok(r) => r,
        Err(err) => return ok(HttpResponse::Ok().json(
            ApiResponse { message: response.err().unwrap().to_string(), data: "", ..Default::default() }))
    };


    let data: Value = response.json().unwrap();
    let errcode = data.get("errcode").unwrap().as_i64().unwrap();
    if errcode != 0 {
        let errmsg = data.get("errmsg").unwrap().as_str().unwrap();
        let ip = req.head().peer_addr.unwrap().ip().to_string();
        warn!("MiniAPP code_session errmsg={},ip={}", errmsg, ip);
        return ok(HttpResponse::Ok().json(ApiResponse { message: errmsg.to_owned(), data: "", ..Default::default() }));
    }

    let session_key = data.get("session_key").unwrap().as_str().unwrap();
    let unionid = data.get("unionid").unwrap().as_str().unwrap();
    if session_key.is_empty() || unionid.is_empty() {
        return ok(HttpResponse::Ok().json(ApiResponse { message: "sessionKey or unionId is empty".to_owned(), data: "", ..Default::default() }));
    }

    let clamis = Claims {
        sub: unionid.to_owned(),
        exp: SEVEN_DAYS,
        iat: Local::now().timestamp_millis() as usize,
        union_id: unionid.to_owned(),
        secret_key: session_key.to_owned(),
    };

    let token = match jwt::encode(&Header::default(), clamis, "xGAdUr0E5i".as_ref()) {
        Ok(t) => t,
        Err(e) => return ok(HttpResponse::Ok().json(ApiResponse { message: e.description().to_owned(), data: "", ..Default::default() })),
    };

    ok(HttpResponse::Ok().json(ApiResponse { data: "", token, ..Default::default() }))
}