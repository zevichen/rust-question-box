use std::collections::HashMap;

use actix_web::{Error as AWError, HttpResponse, web};
use actix_web::error::BlockingError;
use futures::Future;
use futures::future::ok;
use rusqlite::NO_PARAMS;

use crate::model::content::ApiResponse;
use crate::share::common::SqlitePool;
use crate::model::tag::{TagForm, TagInfo};
use crate::utils::tool;
use std::error::Error;

/// tag list
pub fn tag_list(
    form: web::Json<TagForm>,
    pool: web::Data<SqlitePool>,
) -> impl Future<Item=HttpResponse, Error=AWError> {
    web::block(move || {

        if form.token.is_empty(){
            return Err(ApiResponse::fail("token is empty".to_owned(),""));
        }

        let result = tool::jwt_decode(&form.token);
        if result.is_err(){
            return Err(ApiResponse::fail(result.err().unwrap().description().to_owned(), ""));
        }

        let uuid = result.unwrap().claims.sub;

        let conn = pool.get().unwrap();

        let mut stmt = conn.prepare("select id,tag_name from tag where is_delete = 0 and uuid = $1 order by id desc").unwrap();
        match stmt.query_map(&[&uuid], |row| {
            Ok(TagInfo{
                id:row.get_unwrap(0),
                tag_name:row.get_unwrap(1)
            })
        }).and_then(|mapped_rows| {
            Ok(mapped_rows
                .map(|row| row.unwrap())
                .collect::<Vec<TagInfo>>())
        }) {
            Ok(r) => Ok(ApiResponse::success(r)),
            Err(e) => Err(ApiResponse::fail(e.to_string(), ""))
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