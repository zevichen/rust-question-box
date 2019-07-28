#[warn(dead_code)]
extern crate jsonwebtoken as jwt;
#[macro_use]
extern crate serde_derive;

use std::time::Duration;

use chrono::Date;
use chrono::Local;
use jwt::{Algorithm, decode, encode, Header, TokenData, Validation};
use jwt::errors::ErrorKind;

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
/// iss: jwt签发者
/// sub: jwt所面向的用户
/// aud: 接收jwt的一方
/// exp: jwt的过期时间，这个过期时间必须要大于签发时间
/// nbf: 定义在什么时间之前，该jwt都是不可用的.
/// iat: jwt的签发时间
/// jti: jwt的唯一身份标识，主要用来作为一次性token,从而回避重放攻击。
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Claims {
    // uuid
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    // #[validate(length(min = "1", max = "1000000"))]
    pub union_id: String,
    pub session_key: String,
    pub nick_name: String,
}

///
///
///
///
///
///
///
///
fn main() {
    // my_demo();
    //custom_header();
//    validation();

    user_info();
}

fn user_info() {
    let now_second = (Local::now().timestamp_millis() / 1000) as usize;
    println!("current time = {}", now_second);

    let my_claims = Claims {
        sub: "3fEUUjxZmujwX9OONDvwit5fQnikrl3d".to_owned(),
        exp: now_second + 7 * 24 * 60 * 60,
        iat: now_second,
        union_id: "fiwjgprgjrepgjhqfoqj".to_owned(),
        session_key: "session-key".to_owned(),
        nick_name: "zhangsan".to_owned(),
    };

    let secret = "xGAdUr0E5i";

    let token = match encode(&Header::default(), &my_claims, secret.as_bytes()) {
        Ok(t) => t,
        Err(_) => "".to_owned(),
    };
    // eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMjA3MjcyODY5OTE4MDc4MzE1MyIsImV4cCI6MCwiaWF0IjoxNTY0MDM5MTM1MTE4LCJ1bmlvbl9pZCI6ImZpd2pncHJnanJlcGdqaHFmb3FqIiwic2Vzc2lvbl9rZXkiOiJzZXNzaW9uLWtleSIsIm5pY2tfbmFtZSI6InpoYW5nc2FuIn0.RvQdvuBJQ5GR-MRiBLXYxyh91JOYToNo68mQIg07qLc
    println!("token={}", token);

//    std::thread::sleep(Duration::from_secs(2));

    let token_data = decode::<Claims>(&token, secret.as_ref(), &Validation::default()).unwrap();
    println!("{:?}", token_data.claims);
}

fn my_demo() {
    let my_claims = Claims {
        sub: "b@b.com".to_owned(),
        exp: 0,
        iat: 0,
        union_id: "".to_string(),
        session_key: "".to_string(),
        nick_name: "".to_string(),
    };

    let secret = "123456";

    let token = match encode(&Header::default(), &my_claims, secret.as_bytes()) {
        Ok(t) => t,
        Err(_) => "".to_owned(),
    };
    println!("token={}", token);

    let claims: Claims = decode(token.as_str(), secret.as_bytes(), &Validation::default()).ok().unwrap().claims;
    println!("{:?}", claims);
}

fn custom_header() {
    let my_claims = Claims {
        sub: "b@b.com".to_owned(),
        exp: 1000000,
        iat: 0,
        union_id: "".to_string(),
        session_key: "".to_string(),
        nick_name: "".to_string(),
    };
    let key = "secret";

    let mut header = Header::default();
    header.kid = Some("signing_key".to_owned());
    header.alg = Algorithm::HS512;

    let token = match encode(&header, &my_claims, key.as_ref()) {
        Ok(t) => t,
        Err(_) => "".to_owned(),
    };
    println!("{:?}", token);

    let token_data =
        match decode::<Claims>(&token, key.as_ref(), &Validation::new(Algorithm::HS512)) {
            Ok(c) => c,
            Err(err) => match *err.kind() {
                ErrorKind::ExpiredSignature => {
                    println!("expired");
                    return;
                }
                _ => return,
            },
        };

    println!("{:?}", token_data.claims);
    println!("{:?}", token_data.header);
}

fn validation() {
    let my_claims = Claims {
        sub: "b@b.com".to_owned(),
        exp: 10000000000,
        iat: 0,
        union_id: "".to_string(),
        session_key: "".to_string(),
        nick_name: "".to_string(),
    };
    let key = "secret";
    let token = match encode(&Header::default(), &my_claims, key.as_ref()) {
        Ok(t) => t,
        Err(_) => panic!(), // in practice you would return the error
    };

    let validation = Validation { sub: Some("b@b.com".to_string()), ..Validation::default() };
    let token_data = match decode::<Claims>(&token, key.as_ref(), &validation) {
        Ok(c) => c,
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => panic!("Token is invalid"), // Example on how to handle a specific error
            ErrorKind::InvalidIssuer => panic!("Issuer is invalid"), // Example on how to handle a specific error
            ErrorKind::ExpiredSignature => panic!("Token was expired"),
            _ => panic!("Some other errors"),
        },
    };
    println!("{:?}", token_data.claims);
    println!("{:?}", token_data.header);
}
