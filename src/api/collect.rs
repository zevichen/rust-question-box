use std::error::Error;

use actix_web::{Error as AWError, HttpResponse, web};
use actix_web::error::BlockingError;
use futures::Future;
use futures::future::ok;

use crate::model::collect::CollectForm;
use crate::model::content::ApiResponse;
use crate::model::question::QuestionInfo;
use crate::share::code;
use crate::share::{common, common::SqlitePool};
use crate::utils::tool;

/// 收藏
pub fn collect_info(
    mut form: web::Json<CollectForm>,
    pool: web::Data<SqlitePool>,
) -> impl Future<Item=HttpResponse, Error=AWError> {
    web::block(move || {
        if form.page_index <= 0 {
            form.page_index = 0;
        }
        if form.page_size <= 0 {
            form.page_size = common::PAGE_SIZE;
        }

        if form.token.is_empty() {
            return Err(ApiResponse::fail("token is empty".to_owned(), Vec::<QuestionInfo>::new()));
        }

        let token = match tool::jwt_decode(&form.token) {
            Ok(t) => t,
            Err(e) => return Ok(ApiResponse::fail_code(code::REAUTH, e.to_string(), Vec::<QuestionInfo>::new())),
        };

        let page_start = format!("{}", form.page_index * form.page_size);
        let page_size = format!("{}", form.page_size);

        let mut response = ApiResponse::success(Vec::<QuestionInfo>::new());
        response.page_size = form.page_size;
        response.page_index = form.page_index;

        let conn = pool.get().unwrap();
        let mut stmt = conn.prepare("select question_id from collect where is_delete = 0 and uuid = $1 \
        order by gmt_create desc limit $2,$3").unwrap();

        println!("sub = {},page_start = {},page_size = {}", token.claims.sub, page_start, page_size);

        let sr = stmt.query_map(&[&token.claims.sub, &page_start, &page_size],
            |row| {
                let question_id: i32 = row.get_unwrap(0);
                let result = conn.query_row("select id,name,question_image,question_desc,answer_image,answer_desc,\
         degree,question_type,subject_id,subject_name,tags,gmt_create from question where id = $1 and is_delete = 0",
                    &[question_id], |row| {
                        Ok(QuestionInfo {
                            id: row.get_unwrap(0),
                            name: row.get_unwrap(1),
                            question_image: row.get_unwrap(2),
                            question_desc: row.get_unwrap(3),
                            answer_image: row.get_unwrap(4),
                            answer_desc: row.get_unwrap(5),
                            degree: row.get_unwrap(6),
                            question_type: row.get_unwrap(7),
                            subject_id: row.get_unwrap(8),
                            subject_name: row.get_unwrap(9),
                            tags: row.get_unwrap(10),
                            gmt_create: row.get_unwrap(11),
                        })
                    });
                result
            })
            .and_then(|mapped_rows| {
                Ok(mapped_rows.map(|row| row.unwrap()).collect::<Vec<QuestionInfo>>())
            });

        if sr.is_err() {
            return Err(ApiResponse::fail(sr.err().unwrap().description().to_owned(), Vec::<QuestionInfo>::new()));
        }

        response.data = sr.unwrap();

        match conn.query_row("select count(*) from collect where is_delete = 0 and uuid = $1", &[token.claims.sub], |row| {
            response.page_total = row.get_unwrap(0);
            Ok(())
        }) {
            Ok(_) => Ok(response),
            Err(e) => Err(ApiResponse::fail(e.description().to_owned(), Vec::<QuestionInfo>::new())),
        }
    }).then(|res| match res {
        Ok(r) => ok(HttpResponse::Ok().json(r)),
        Err(e) => match e {
            BlockingError::Error(e) => ok(HttpResponse::Ok().json(e)),
            BlockingError::Canceled => ok(HttpResponse::Ok()
                .json(ApiResponse::fail("system error".to_owned(), Vec::<QuestionInfo>::new())))
        },
    })
}

/// 用户收藏
pub fn collect_add(
    form: web::Json<CollectForm>,
    pool: web::Data<SqlitePool>,
) -> impl Future<Item=HttpResponse, Error=AWError> {
    web::block(move || {
        if form.token.is_empty() || form.question_id <= 0 {
            return Err(ApiResponse::fail("paramaters is empty".to_owned(), 0));
        }

        let token = match tool::jwt_decode(&form.token) {
            Ok(t) => t,
            Err(e) => return Ok(ApiResponse::fail_code(code::REAUTH, e.to_string(), 0)),
        };

        let question_id = format!("{}", form.question_id);

        let conn = pool.get().unwrap();
        match conn.execute("insert or replace into collect(uuid,question_id,gmt_create,gmt_modified) values($1,$2,$3,$4)"
            , &[&token.claims.sub, &question_id]) {
            Ok(_) => Ok(ApiResponse::success(conn.last_insert_rowid())),
            Err(e) => Err(ApiResponse::fail(e.description().to_owned(), 0))
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