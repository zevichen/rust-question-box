use std::collections::HashMap;

use actix_web::{Error, HttpResponse, web};
use actix_web::error::BlockingError;
use futures::Future;
use futures::future::ok;
use rusqlite::NO_PARAMS;

use crate::model::content::ApiResponse;
use crate::share::common::SqlitePool;

#[derive(Debug, Serialize, Deserialize)]
pub enum SubjectAgg {
    SubjectList { id: i64, subject_name: String }
}

/// suject list
pub fn subject_list(
    pool: web::Data<SqlitePool>,
) -> impl Future<Item=HttpResponse, Error=Error> {
    web::block(move || {
        let conn = pool.get().unwrap();

        let mut stmt = conn.prepare("select id,subject_name from subject where is_delete = 0").unwrap();
        match stmt.query_map(NO_PARAMS, |row| {
            let mut map = HashMap::new();
            let id: i64 = row.get(0).unwrap();
            map.insert("id", format!("{}", id));
            map.insert("subject_name", row.get(1).unwrap());
            Ok(map)
        }).and_then(|mapped_rows| {
            Ok(mapped_rows
                .map(|row| row.unwrap())
                .collect::<Vec<HashMap<&str, String>>>())
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