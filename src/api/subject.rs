use std::collections::HashMap;

use actix_web::{Error, HttpResponse, web};
use actix_web::error::BlockingError;
use futures::Future;
use futures::future::ok;
use hyper::server::conn::Http;
use rusqlite::NO_PARAMS;

use crate::model::content::ApiResponse;
use crate::share::common::SqlitePool;

/// suject list
pub fn subject_list(
    pool: web::Data<SqlitePool>,
) -> impl Future<Item=HttpResponse, Error=Error> {
    web::block(move || {
        let mut v: Vec<HashMap<&str, String>> = Vec::new();
        let conn = pool.get().unwrap();

        let mut stmt = conn.prepare("select id,subject_name from subject where is_delete = 0").unwrap();
        let result = match stmt.query_map(NO_PARAMS, |row| {
            let mut map: HashMap<&str, String> = HashMap::new();
            let id: i64 = row.get("id").unwrap();
            map.insert("id", format!("{}", id));
            map.insert("subject_name", row.get("subject_name").unwrap());
            Ok(map)
        }) {
            Ok(m) => m,
            Err(e) => return Err(ApiResponse::fail(e.to_string(), v))
        };

        for r in result {
            v.push(r.unwrap());
        }

        Ok(ApiResponse::success(v))
    }).then(|res| match res {
        Ok(r) => ok(HttpResponse::Ok().json(r)),
        Err(e) => match e {
            BlockingError::Error(e) => ok(HttpResponse::Ok().json(e)),
            BlockingError::Canceled => ok(HttpResponse::Ok()
                .json(ApiResponse::fail("system error".to_owned(), "")))
        },
    })
}