#[warn(unused_must_use)]
#[warn(dead_code)]
extern crate actix;
extern crate actix_files;
extern crate actix_http;
extern crate actix_session;
extern crate actix_web;
extern crate bcrypt;
extern crate chrono;
extern crate core;
extern crate dotenv;
extern crate env_logger;
extern crate futures;
extern crate jsonwebtoken as jwt;
#[macro_use]
extern crate log;
extern crate md5;
extern crate num_cpus;
extern crate openssl;
extern crate pulldown_cmark;
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rand_core;
extern crate regex;
extern crate ring;
#[macro_use]
extern crate serde;
extern crate serde_json;
extern crate timeago;
extern crate uuid;
extern crate reqwest;
extern crate actix_identity;
#[warn(dead_code)]
extern crate jsonwebtoken as jwt;
use chrono::prelude::*;

use std::{env, io};

use actix_web::{App, HttpServer, middleware, web};

use api::*;
use actix_identity::{IdentityService, CookieIdentityPolicy};

mod api;
mod share;
mod utils;
mod db;
mod model;

// session expired 7 days.
const SEVEN_DAYS: i64 = 7 * 24 * 60 * 60;

fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    dotenv::dotenv().ok();

    let sys = actix_rt::System::new("question-box");

    let pool = db::init();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(IdentityService::new(
                // <- create identity middleware
                CookieIdentityPolicy::new(&[0; 32])    // <- create cookie identity policy
                    .name("auth-cookie")
                    .secure(false)))// true.That need to use https request.
            .wrap(middleware::Logger::default())
            .route("/favicon.ico", web::get().to_async(home::favicon))
            .route("/", web::get().to_async(home::index))
            .route("/login", web::get().to_async(home::login))
            .route("/logout", web::get().to_async(home::logout))
            .service(
                web::scope("/user")
                    .route("/login", web::post().to_async(user::login))
                    .route("/logout", web::get().to_async(user::logout))
                    .route("/info", web::get().to_async(user::info))
            )
    }).bind("localhost:8080").unwrap().shutdown_timeout(5)
        .start();

    println!("Starting http server: 127.0.0.1:8080");
    sys.run()
}
