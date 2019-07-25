use actix_web::{Error, HttpResponse, web};
use actix_web::error::BlockingError;
use futures::Future;
use futures::future::ok;
use jwt::Validation;
use r2d2_sqlite::SqliteConnectionManager;

use crate::model::content::{ApiRequest, ApiResponse};
use crate::model::token::Claims;
use crate::model::user::UserInfo;
use crate::share::code;

type SqlitePool = r2d2::Pool<SqliteConnectionManager>;

//https://docs.rs/actix-identity/0.1.0/actix_identity/
/// user_info
pub fn info(
    item: web::Json<ApiRequest>,
    pool: web::Data<SqlitePool>,
) -> impl Future<Item=HttpResponse, Error=Error> {
    web::block(move || {
        let mut user_info = UserInfo::default();
        if item.token.is_empty() {
            return Err(ApiResponse::fail("token is empty".to_owned(), user_info));
        }

        let jwt_secret = std::env::var("JWT_SECRET").expect("jwt_secret not exist");
        let token = match jwt::decode::<Claims>(&item.token, jwt_secret.as_ref(), &Validation::default()) {
            Ok(t) => t,
            Err(e) => return Ok(ApiResponse::fail_code(code::REAUTH, e.to_string(), user_info)),
        };

        let union_id = token.claims.union_id;
        let conn = pool.get().unwrap();

        conn.query_row("select id,nick_name,mobile,icon from user where union_id = $1 and is_delete = 0 limit 1", &[&union_id], |r| {
            user_info.id = r.get_unwrap("id");
            user_info.nick_name = r.get_unwrap("nick_name");
            user_info.icon = r.get_unwrap("icon");
            Ok(())
        }).expect("select error.");

        Ok(ApiResponse::success(user_info))
    }).then(|res| match res {
        Ok(r) => ok(HttpResponse::Ok().json(r)),
        Err(e) => match e {
            BlockingError::Error(e) => ok(HttpResponse::Ok().json(e)),
            BlockingError::Canceled => ok(HttpResponse::Ok()
                .json(ApiResponse::fail("system error".to_owned(), UserInfo::default())))
        },
    })
}

// logout
//pub fn logout(session: Session) -> impl Future<Item=HttpResponse, Error=Error> {
//    session.clear();
//    ok(HttpResponse::Ok().json(ApiResponse { message: "logout".to_owned(), data: "", ..Default::default() }))
//}
