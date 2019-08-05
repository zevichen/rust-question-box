use std::error::Error;

use actix_web::{Error as AWError, HttpResponse, web};
use actix_web::error::BlockingError;
use futures::{Future, future::ok};

use crate::model::content::{ApiRequest, ApiResponse};
use crate::model::home::{HomeInfo, HomeSubjectInfo};
use crate::share::code;
use crate::share::common::SqlitePool;
use crate::utils::tool;

/// index
pub fn index(
    item: web::Json<ApiRequest>,
    pool: web::Data<SqlitePool>,
) -> impl Future<Item=HttpResponse, Error=AWError> {
    web::block(move || {
        if item.token.is_empty() {
            return Err(ApiResponse::fail_code(code::REAUTH, "token is empty".to_owned(), HomeInfo::default()));
        }

        let token_data = match tool::jwt_decode(&item.token) {
            Ok(r) => r,
            Err(e) => return Err(ApiResponse::fail_code(code::REAUTH, e.description().to_owned(), HomeInfo::default())),
        };

        let conn = pool.get().unwrap();
        let mut stmt = conn.prepare("select subject_id,subject_name,count(subject_id) from question \
        where uuid = $1 and is_delete = 0 group by subject_id").unwrap();

        let uuid = token_data.claims.sub;
        println!("uuid = {}", uuid);

        let result = stmt.query_map(&[&uuid], |row| {
            Ok(HomeSubjectInfo {
                id: row.get_unwrap(0),
                subject_name: row.get_unwrap(1),
                count: row.get_unwrap(2),
            })
        }).and_then(|mapped_rows| {
            Ok(mapped_rows.map(|row| row.unwrap()).collect::<Vec<HomeSubjectInfo>>())
        });

        if result.is_err() {
            return Err(ApiResponse::fail(result.err().unwrap().description().to_owned(), HomeInfo::default()));
        };

        let mut home_info = HomeInfo::default();
        home_info.subject_infos = result.ok().unwrap();
        Ok(ApiResponse::success(home_info))
    }).then(|res| match res {
        Ok(r) => ok(HttpResponse::Ok().json(r)),
        Err(e) => match e {
            BlockingError::Error(e) => ok(HttpResponse::Ok().json(e)),
            BlockingError::Canceled => ok(HttpResponse::Ok()
                .json(ApiResponse::fail("Thread pool is gone".to_owned(), "")))
        },
    })
}


/// favicon api
pub fn favicon() -> impl Future<Item=HttpResponse, Error=AWError> {
    ok(HttpResponse::Ok().finish())
}

