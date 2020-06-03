use crate::{models::{user_token::{JWTToken}}};

use std::env;
use jsonwebtoken::{DecodingKey, TokenData, Validation};
pub fn decode_token(token: String) -> jsonwebtoken::errors::Result<TokenData<JWTToken>> {
    let secret: String = env::var("JWT_SECRET").expect("Missing JWT_SECRET in .env");
    jsonwebtoken::decode::<JWTToken>(&token, &DecodingKey::from_base64_secret(&secret).expect("Failed Creating encoding key from base 64"), &Validation::default())
}

pub fn verify_token(token: String) -> Result<String, String> {
    let result = decode_token(token);
    match result {
        Ok(token_data) => Ok(token_data.claims.user_id.to_string()),
        Err(_) => Err("Invalid token".to_string())
    }
}
