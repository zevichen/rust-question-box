use std::error::Error;

use actix_web::{Error as AWError, HttpResponse, web};
use actix_web::error::BlockingError;
use futures::Future;
use futures::future::ok;

use crate::model::content::{ApiRequest, ApiResponse};
use crate::model::token::Claims;
use crate::model::user::UserInfo;
use crate::share::code;
use crate::share::common::SqlitePool;
use crate::utils::tool;

//https://docs.rs/actix-identity/0.1.0/actix_identity/
/// user_info
pub fn info(
    item: web::Json<ApiRequest>,
    pool: web::Data<SqlitePool>,
) -> impl Future<Item=HttpResponse, Error=AWError> {
    web::block(move || {
        let default = UserInfo::default();
        if item.token.is_empty() {
            return Err(ApiResponse::fail("token is empty".to_owned(), default));
        }

        let token = match tool::jwt_decode(&item.token) {
            Ok(t) => t,
            Err(e) => return Ok(ApiResponse::fail_code(code::REAUTH, e.to_string(), default)),
        };

        let union_id = token.claims.union_id;
        let conn = pool.get().unwrap();

        match conn.query_row("select id,uuid,user_name,nick_name,icon from user where union_id = $1 and is_delete = 0 limit 1",
            &[&union_id], |r|
                Ok(UserInfo {
                    id: r.get_unwrap(0),
                    uuid: r.get_unwrap(1),
                    user_name: r.get_unwrap(2),
                    nick_name: r.get_unwrap(3),
                    icon: r.get_unwrap(4),
                }),
        ) {
            Ok(r) => Ok(ApiResponse::success(r)),
            Err(e) => Err(ApiResponse::fail(e.description().to_owned(), default))
        }
    }).then(|res| match res {
        Ok(r) => ok(HttpResponse::Ok().json(r)),
        Err(e) => match e {
            BlockingError::Error(e) => ok(HttpResponse::Ok().json(e)),
            BlockingError::Canceled => ok(HttpResponse::Ok()
                .json(ApiResponse::fail("system error".to_owned(), UserInfo::default())))
        },
    })
}

//
pub fn is_login(
    item: web::Json<ApiRequest>
) -> impl Future<Item=HttpResponse, Error=AWError> {
    web::block(move || {
        if item.token.is_empty() {
            return Err(ApiResponse::fail("unlogin".to_owned(), ""));
        }

        let jwt_secret = std::env::var("JWT_SECRET").unwrap();
        let result = jwt::decode::<Claims>(&item.token, jwt_secret.as_ref(), &jwt::Validation::default());
        if result.is_err() {
            return Err(ApiResponse::fail(result.err().unwrap().to_string(), ""));
        }

        Ok(ApiResponse::success(result.unwrap().claims.sub))
    }).then(|res| match res {
        Ok(r) => ok(HttpResponse::Ok().json(r)),
        Err(e) => match e {
            BlockingError::Error(e) => ok(HttpResponse::Ok().json(e)),
            BlockingError::Canceled => ok(HttpResponse::Ok()
                .json(ApiResponse::fail("system error".to_owned(), "")))
        },
    })
}