use actix_http::Error;
use actix_web::{HttpResponse, web};
use futures::{Future, future::ok};
use r2d2_sqlite::SqliteConnectionManager;

use crate::model::content::{ApiRequest, ApiResponse};
use crate::model::token::Claims;
use crate::share::code;

type SqlitePool = r2d2::Pool<SqliteConnectionManager>;

/// index
pub fn index(
    item: web::Json<ApiRequest>,
    pool: web::Data<SqlitePool>,
) -> impl Future<Item=HttpResponse, Error=Error> {
    web::block(move || {
        if item.token.is_empty() {
            return Err(ApiResponse { code: code::FAILED, message: "token is empty".to_owned(), data: "".to_owned(), ..Default::default() });
        }

        use jwt::errors::ErrorKind;
        let secret = std::env::var("SECRET").unwrap();
        let tokenData = match jwt::decode::<Claims>(&item.token, secret.as_ref(), &jwt::Validation::default()) {
            Ok(r) => r,
            Err(e) => match *e.kind() {
                ErrorKind::ExpiredSignature => return Ok(ApiResponse { code: code::REAUTH, message: "token has expired".to_owned(), data: "".to_owned(), ..Default::default() }),
                _ => return Err(ApiResponse { code: code::FAILED, message: e.to_string(), data: "".to_owned(), ..Default::default() }),
            }
        };

        Ok(ApiResponse { data: "".to_owned(), ..Default::default() })
    }).then(|res| match res {
        Ok(r) => Ok(HttpResponse::Ok().json(&r)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}


/// favicon api
pub fn favicon() -> impl Future<Item=HttpResponse, Error=Error> {
    ok(HttpResponse::Ok().finish())
}

