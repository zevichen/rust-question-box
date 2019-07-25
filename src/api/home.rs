use actix_http::Error;
use actix_web::{HttpResponse, web};
use actix_web::error::BlockingError;
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
            return Err(ApiResponse::fail_code(code::REAUTH, "token is empty".to_owned(),""));
        }

        use jwt::errors::ErrorKind;
        let jwt_secret = std::env::var("JWT_SECRET").unwrap();
        let token_data = match jwt::decode::<Claims>(&item.token, jwt_secret.as_ref(), &jwt::Validation::default()) {
            Ok(r) => r,
            Err(e) => return Err(ApiResponse::fail_code(code::REAUTH, e.to_string(),"")),
        };

        Ok(ApiResponse::success(token_data.claims.sub))
    }).then(|res| match res {
        Ok(r) => ok(HttpResponse::Ok().json(r)),
        Err(e) => match e {
            BlockingError::Error(e) => ok(HttpResponse::Ok().json(e)),
            BlockingError::Canceled => ok(HttpResponse::Ok()
                .json(ApiResponse::fail("Thread pool is gone".to_owned(),"")))
        },
    })
}


/// favicon api
pub fn favicon() -> impl Future<Item=HttpResponse, Error=Error> {
    ok(HttpResponse::Ok().finish())
}

