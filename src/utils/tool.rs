use std::env;
use jwt::{decode, Validation};

use crate::model::token::Claims;
use jsonwebtoken::errors::Result;
use jsonwebtoken::TokenData;

pub fn jwt_decode(token: &str) -> Result<TokenData<Claims>> {
    let secret = env::var("JWT_SECRET").expect("failed to get secret");
    let result = decode::<Claims>(token, secret.as_ref(), &Validation::default());
    result
}