use std::collections::HashMap;

use actix_web::{Error as AWError, HttpResponse, web};
use actix_web::error::BlockingError;
use futures::Future;
use futures::future::ok;
use rusqlite::NO_PARAMS;

use crate::model::content::ApiResponse;
use crate::share::common::SqlitePool;
use crate::model::subject::SubjectInfo;

/// subject list
pub fn subject_list(
    pool: web::Data<SqlitePool>,
) -> impl Future<Item=HttpResponse, Error=AWError> {
    web::block(move || {
        let conn = pool.get().unwrap();

        let mut stmt = conn.prepare("select id,subject_name from subject where is_delete = 0").unwrap();
        match stmt.query_map(NO_PARAMS, |row| {
            Ok(SubjectInfo{
                id: row.get_unwrap(0),
                subject_name: row.get_unwrap(1)
            })
        }).and_then(|mapped_rows| {
            Ok(mapped_rows
                .map(|row| row.unwrap())
                .collect::<Vec<SubjectInfo>>())
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