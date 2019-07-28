use std::fs;
use std::io::Write;

use actix_multipart::{Field, Multipart, MultipartError};
use actix_web::{error, Error, HttpResponse, web};
use actix_web::error::BlockingError;
use chrono::Local;
use futures::{Future, Stream};
use futures::future::{Either, err, ok};
use rand::distributions::Alphanumeric;
use rand::Rng;

use crate::model::content::ApiResponse;
use crate::model::question::QuestionForm;
use crate::share::{common, common::SqlitePool};
use crate::share::code;
use crate::utils::tool;

/// add question
pub fn add_question(
    form: web::Json<QuestionForm>,
    pool: web::Data<SqlitePool>,
) -> impl Future<Item=HttpResponse, Error=Error> {
    web::block(move || {
        if form.token.is_empty() {
            return Err(ApiResponse::fail("token is empty".to_owned(), ""));
        }

        let claims = match tool::jwt_decode(form.token.as_ref()) {
            Ok(r) => r.claims,
            Err(e) => return Err(ApiResponse::fail_code(code::REAUTH, e.to_string(), ""))
        };


        let now = Local::now().format(common::COMMON_DATA_FORMAT).to_string();
        let mut conn = pool.get().expect("Failed to get connection");

        let tx = conn.transaction().unwrap();

        if !form.tags.is_empty() {
            let tags: Vec<&str> = form.tags.split(",").collect();
            if !tags.is_empty() {
                let mut stmt = tx.prepare("insert or replace into tag(tag_name,uuid,gmt_create,gmt_modified) values (?,?,?,?)")
                    .expect("tx open stmt failed.");
                for tag in tags {
                    if !tag.is_empty(){
                        stmt.execute(&[tag, &claims.sub, &now, &now]).expect("stmt insert or replace failed");
                    }
                }
            }
        }

        tx.execute(
            "insert into question (uuid,name,question_image,question_desc,answer_image,answer_desc,degree,question_type,\
            subject_id,subject_name,tags,is_delete,gmt_create,gmt_modified) \
            values ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,0,$12,$13)",
            &[&claims.sub, &form.name, &form.question_image, &form.question_desc, &form.answer_image, &form.answer_desc,
                &form.degree, &form.question_type, &form.subject_id, &form.subject_name, &form.tags, &now, &now],
        ).expect("question insertion failed.");

        tx.commit();

        Ok(ApiResponse::success(conn.last_insert_rowid()))
    }).then(|res| match res {
        Ok(r) => ok(HttpResponse::Ok().json(r)),
        Err(e) => match e {
            BlockingError::Error(e) => ok(HttpResponse::Ok().json(e)),
            BlockingError::Canceled => ok(HttpResponse::Ok()
                .json(ApiResponse::fail("system error".to_owned(), "")))
        },
    })
}

/// upload image
pub fn upload_image(
    multipart: Multipart,
) -> impl Future<Item=HttpResponse, Error=Error> {
    multipart
        .map_err(error::ErrorInternalServerError)
        .map(|field| save_file(field).into_stream())
        .flatten()
        .collect()
        .map(|names| {
            let now = Local::now().format(common::COMMON_DATA_FORMAT).to_string();
            info!("{} method:upload_image names:{:?}", now, names);
            HttpResponse::Ok().json(ApiResponse::success(names))
        })
        .map_err(|e| {
            println!("{:?}", e);
            e
        })
}

/// 保存文件
pub fn save_file(field: Field) -> impl Future<Item=String, Error=Error> {
    let mut file_name = rand::thread_rng().sample_iter(&Alphanumeric).take(30).collect::<String>();
    file_name.push_str(".jpg");
    let file_path_string = format!("static/{}", file_name);
    let file = match fs::File::create(file_path_string) {
        Ok(file) => file,
        Err(e) => return Either::A(err(error::ErrorInternalServerError(e))),
    };

    Either::B(
        field.fold((file, 0i64, file_name), move |(mut file, mut acc, file_name), bytes| {
            // fs operations are blocking, we have to execute writes
            // on threadpool
            web::block(move || {
                file.write_all(bytes.as_ref()).map_err(|e| {
                    println!("file.write_all failed: {:?}", e);
                    MultipartError::Payload(error::PayloadError::Io(e))
                })?;
                acc += bytes.len() as i64;
                Ok((file, acc, file_name))
            }).map_err(|e: error::BlockingError<MultipartError>| {
                match e {
                    error::BlockingError::Error(e) => e,
                    error::BlockingError::Canceled => MultipartError::Incomplete,
                }
            })
        })
            .map(|(_, acc, file_name)| {
                info!("method=save_file name={} size={}", file_name, acc);
                file_name
            })
            .map_err(|e| {
                println!("save_file failed, {:?}", e);
                error::ErrorInternalServerError(e)
            }),
    )
}
