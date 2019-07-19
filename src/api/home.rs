use actix_http::{Error, http::StatusCode};
use actix_session::Session;
use actix_web::HttpResponse;
use futures::{Future, future::ok};

const COUNTER: &str = "counter";

/// index
pub fn index( session: Session) -> impl Future<Item=HttpResponse, Error=Error> {
    // session
    if let Some(count) = session.get::<i32>(COUNTER).unwrap() {
        println!("session.counter = {}", count);
        session.set(COUNTER, 1 + count).unwrap();
    } else {
        session.set(COUNTER, 1).unwrap();
    };

    ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body("hello"))
}

/// favicon api
pub fn favicon() -> impl Future<Item=HttpResponse, Error=Error> {
    ok(HttpResponse::build(StatusCode::OK).body(""))
}