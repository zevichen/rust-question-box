extern crate jsonwebtoken as jwt;
#[macro_use]
extern crate serde_derive;

use jwt::{Algorithm, decode, encode, Header, Validation};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}
fn main() {


    let my_claims = Claims {
        sub: "b@b.com".to_owned(),
        company: "ACME".to_owned(),
        exp: 0
    };
    let token = encode(&Header::default(), &my_claims, "secret".as_ref()).ok().unwrap();
    println!("token={}", token);


}

