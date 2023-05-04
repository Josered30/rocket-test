use crate::cores::errors::ApiError;

use chrono::{Duration, Local};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub email: String,
    pub id: i32,
    pub exp: usize,
}

impl Claims {
    fn new(email: &str, id: i32) -> Self {
        Claims {
            email: email.to_string(),
            id: id.clone(),
            exp: (Local::now() + Duration::hours(24)).timestamp() as usize,
        }
    }
}

pub fn create_token(email: &str, id: i32) -> Result<String, ApiError> {
    let claims = Claims::new(email, id);
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_secret().as_bytes()),
    )
    .map_err(|error| ApiError::new(401, format!("Token error: {}", error)))
}

pub fn decode_token(token: &str) -> Result<Claims, ApiError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(get_secret().as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|error| ApiError::new(401, format!("Token error: {}", error)))
}

fn get_secret() -> String {
    dotenv::var("JWT_SECRET").unwrap()
}
