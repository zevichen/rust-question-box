use std::cell::Cell;
use std::fs;
use std::io::Write;

use actix_multipart::{Field, Multipart, MultipartError};
use actix_web::{App, error, Error, HttpResponse, HttpServer, middleware, web};
use actix_web::error::BlockingError;
use futures::{Future, Sink, Stream};
use futures::future::{Either, err, ok};

use crate::model::content::ApiResponse;
use crate::model::question::QuestionForm;

//pub fn add_question(
//    multipart: Multipart,
//    form: web::Json<QuestionForm>,
//    _: web::Data<SqlitePool>,
//) -> impl Future<Item=HttpResponse, Error=Error> {
//    web::block(move || {
////        multipart.map_err(error::ErrorInternalServerError)
////            .map(|field| {
////                println!("{:?}", field);
////                Ok(())
////            }).close()
//    }).then(|res| match res {
//        Ok(r) => ok(HttpResponse::Ok().json(r)),
//        Err(e) => match e {
//            BlockingError::Error(e) => ok(HttpResponse::Ok().json(e)),
//            BlockingError::Canceled => ok(HttpResponse::Ok()
//                .json(ApiResponse::fail("Thread pool is gone".to_owned(), "")))
//        },
//    })
//}

/// upload image
pub fn upload_image(
    multipart: Multipart,
) -> impl Future<Item=HttpResponse, Error=Error> {

    multipart
        .map_err(error::ErrorInternalServerError)
        .map(|field| save_file(field).into_stream())
        .flatten()
        .collect()
        .map(|sizes| HttpResponse::Ok().json(sizes))
        .map_err(|e| {
            println!("failed: {}", e);
            e
        })
}

pub fn save_file(field: Field) -> impl Future<Item=i64, Error=Error> {

    let file_path_string = "upload.png";
    let file = match fs::File::create(file_path_string) {
        Ok(file) => file,
        Err(e) => return Either::A(err(error::ErrorInternalServerError(e))),
    };
    Either::B(
        field
            .fold((file, 0i64), move |(mut file, mut acc), bytes| {
                // fs operations are blocking, we have to execute writes
                // on threadpool
                web::block(move || {
                    file.write_all(bytes.as_ref()).map_err(|e| {
                        println!("file.write_all failed: {:?}", e);
                        MultipartError::Payload(error::PayloadError::Io(e))
                    })?;
                    acc += bytes.len() as i64;
                    Ok((file, acc))
                })
                    .map_err(|e: error::BlockingError<MultipartError>| {
                        match e {
                            error::BlockingError::Error(e) => e,
                            error::BlockingError::Canceled => MultipartError::Incomplete,
                        }
                    })
            })
            .map(|(_, acc)| acc)
            .map_err(|e| {
                println!("save_file failed, {:?}", e);
                error::ErrorInternalServerError(e)
            }),
    )
}
