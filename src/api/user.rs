use futures::Future;
use futures::future::ok;
use jwt::Validation;
use r2d2_sqlite::SqliteConnectionManager;

use crate::model::content::{ApiRequest, ApiResponse};
use crate::model::token::Claims;
use crate::model::user::UserInfo;
use crate::share::code;
use actix_web::{web, HttpResponse, ResponseError, Error};

type SqlitePool = r2d2::Pool<SqliteConnectionManager>;

//https://docs.rs/actix-identity/0.1.0/actix_identity/
/// user_info
pub fn info(
    item: web::Json<ApiRequest>,
    pool: web::Data<SqlitePool>,
) -> impl Future<Item=HttpResponse, Error=Error> {
    web::block(move || {
        let mut user_info: UserInfo = Default::default();
        if item.token.is_empty() {
            return Err(ApiResponse { code: code::FAILED, message: "token is empty".to_owned(), ..Default::default() });
        }

        use jwt::errors::ErrorKind;
        let token = match jwt::decode::<Claims>(&item.token, "".as_ref(), &Validation::default()) {
            Ok(t) => t,
            Err(e) => match *e.kind() {
                ErrorKind::ExpiredSignature => return Ok(ApiResponse { code: code::REAUTH, message: "relogin".to_owned(), ..Default::default() }),
                _ => return Err(ApiResponse { code: code::FAILED, message: e.to_string(), ..Default::default() }),
            }
        };

        let union_id = token.claims.union_id;
        let conn = pool.get().unwrap();

        conn.query_row("select id,nick_name,mobile,icon from user where union_id = $1 and is_delete = 0 limit 1", &[&union_id], |r| {
            user_info.id = r.get_unwrap("id");
            user_info.nick_name = r.get_unwrap("nick_name");
            user_info.icon = r.get_unwrap("icon");
            Ok(())
        }).expect("select error.");

        let data = serde_json::to_string(&user_info).expect("serde user_info error.");
        Ok(ApiResponse { data, ..Default::default() })
    }).then(|res| match res {
        Ok(r) => ok(HttpResponse::Ok().json(r)),
        Err(e) => ok(e.render_response()),
    })
}

// logout
//pub fn logout(session: Session) -> impl Future<Item=HttpResponse, Error=Error> {
//    session.clear();
//    ok(HttpResponse::Ok().json(ApiResponse { message: "logout".to_owned(), data: "", ..Default::default() }))
//}
