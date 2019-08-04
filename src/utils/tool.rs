use std::env;
use jwt::{decode, Validation};
use crate::model::token;

use crate::model::token::Claims;
use jsonwebtoken::errors::Result;
use jsonwebtoken::TokenData;
use std::error::Error;

pub fn jwt_decode(token: &str) -> Result<TokenData<Claims>> {
    let secret = env::var("JWT_SECRET").expect("failed to get secret");
    let result = decode::<Claims>(token, secret.as_ref(), &Validation::default());
    result
}

pub fn jwt_encode(
    clamis:token::Claims
)-> Result<String>{
    let jwt_secret = std::env::var("JWT_SECRET").unwrap();
    let token = jwt::encode(&jwt::Header::default(), &clamis, jwt_secret.as_ref()).unwrap();
    Ok(token)
}