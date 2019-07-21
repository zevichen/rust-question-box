use actix_http::{Error, http::StatusCode};
use actix_identity::Identity;
use actix_web::HttpResponse;
use futures::{Future, future::ok};

use crate::model::content::ApiResponse;

/// index
pub fn index(id: Identity) -> impl Future<Item=HttpResponse, Error=Error> {
    // access request identity
    if let Some(id) = id.identity() {
        ok(HttpResponse::Ok().json(ApiResponse { data: format!("Welcome! {}", id), message: "", ..Default::default() }))
    } else {
        ok(HttpResponse::Ok().json(ApiResponse { data: "", message: "Welcome Anonymous!", ..Default::default() }))
    }
}

/// favicon api
pub fn favicon() -> impl Future<Item=HttpResponse, Error=Error> {
    ok(HttpResponse::build(StatusCode::OK).body(""))
}