use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::error::{CustomError, log_error};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenPayload {
    pub account: String,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Claims {
    pub account: String,
    pub iat: i64,
    pub exp: i64,
}

const SECRET: &str = "Xp3#Wq!nLz@d8Tu$k0R&mNvYb1h*";
pub fn parse_token(token: String) -> Result<TokenPayload, CustomError> {
    let decoding_key = DecodingKey::from_secret(SECRET.as_bytes());

    let claims = jsonwebtoken::decode::<Claims>(&token, &decoding_key, &Validation::default())
        .map_err(|error| {
            log_error(CustomError::JWT(format!(
                "JWT: {},{}",
                error.to_string(),
                "parse token error"
            )))
        })?;
    // let exp_result = parse_result.map(|claims| claims.claims.exp);
    let data = claims.claims;
    let exp = data.exp;
    // if let Ok(exp) = exp_result {
    let now = Utc::now().timestamp();
    if now < exp {
        return Ok(TokenPayload {
            account: data.account,
      
        });
    } else {
        return Err(log_error(CustomError::JWT(format!("token expired"))));
    }
}

pub fn generate_token(payload: TokenPayload) -> Result<String, CustomError> {
    let iat = Utc::now().timestamp();
    let exp = iat + 7 * 24 * 60 * 60;
    let claims: Claims = Claims {
        account: payload.account,
        iat,
        exp,
    };
    let encoding_key = EncodingKey::from_secret(SECRET.as_bytes());
    let token = jsonwebtoken::encode(&Header::new(Algorithm::HS256), &claims, &encoding_key)
        .map_err(|error| {
            log_error(CustomError::JWT(format!(
                "JWT: {},{}",
                error.to_string(),
                "generate token error"
            )))
        })?;
    return Ok(token);
}
