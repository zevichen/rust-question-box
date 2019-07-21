extern crate frank_jwt;
#[macro_use]
extern crate serde_json;


use frank_jwt::{Algorithm, decode, encode};

fn main() {
//HS256
    let mut payload = json!({
        "key1": "val1",
        "key2": "val2"
    });

    let mut header = json!({});
    let secret = "secret123";
    let jwt = encode(header, &secret.to_string(), &payload, Algorithm::HS256);

    //RS256
    use std::env;

    let mut payload = json!({
        "key1": "val1",
        "key2": "val2"
    });

    let mut header = json!({});
    let mut keypath = env::current_dir().unwrap();
    keypath.push("some_folder");
    keypath.push("my_rsa_2048_key.pem");
    let jwt = encode(header, &keypath.to_path_buf(), &payload, Algorithm::RS256);
    let (header, payload) = decode(&jwt.unwrap(), &keypath.to_path_buf(), Algorithm::RS256).unwrap();
    println!("header={:?},payload={:?}", header,payload);
}

