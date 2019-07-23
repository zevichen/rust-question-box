use actix_http::{http::StatusCode,Error};
use actix_identity::Identity;
use actix_web::{HttpResponse, web};
use futures::{Future, future::ok};

use crate::model::content::{ApiRequest, ApiResponse};
use crate::model::token::Claims;
use crate::share::code;
use r2d2_sqlite::SqliteConnectionManager;

type SqlitePool = r2d2::Pool<SqliteConnectionManager>;

/// index
pub fn index(
    item: web::Json<ApiRequest>,
    pool: web::Data<SqlitePool>,
)  -> impl Future<Item = HttpResponse, Error = Error>  {
    web::block(move || {
        if item.token.is_empty() {
            return Ok(ApiResponse { code: code::FAILED, message: "token is empty".to_owned(), ..Default::default() });
        }

        use jwt::errors::ErrorKind;
        let secret = std::env::var("SECRET").unwrap();
        let tokenData = match jwt::decode::<Claims>(&item.token, secret.as_ref(), &jwt::Validation::default()) {
            Ok(r) => r,
            Err(e) => match *e.kind() {
                ErrorKind::ExpiredSignature => return Ok(ApiResponse { code: code::REAUTH, message: "token has expired".to_owned(), data: "", ..Default::default() }),
                _ => return Ok(ApiResponse { code: code::FAILED, message: e.to_string(), data: "", ..Default::default() }),
            }
        };

        Ok(ApiResponse { ..Default::default() })
    }).then(|res| match res {
        Ok(r) => ok(HttpResponse::Ok().json(&r)),
        Err(e) => ok(HttpResponse::Ok().json(ApiResponse { message: e.to_string(), data: "", ..Default::default() })),
    })
}


pub fn login(id: Identity) -> impl Future<Item=HttpResponse, Error=Error> {
    id.remember("User1".to_owned()); // <- remember identity
    ok(HttpResponse::Ok().json(ApiResponse { data: "", ..Default::default() }))
}

pub fn logout(id: Identity) -> impl Future<Item=HttpResponse, Error=Error> {
    id.forget();                      // <- remove identity
    ok(HttpResponse::Ok().json(ApiResponse { data: "", ..Default::default() }))
}

/// favicon api
pub fn favicon() -> impl Future<Item=HttpResponse, Error=Error> {
    ok(HttpResponse::build(StatusCode::OK).body(""))
}

