use rocket::{
    http::{ContentType, Status},
    response::{self, Responder},
    Request, Response,
};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::errors::StoreError;

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiError {
    pub status_code: u16,
    pub message: String,
}

impl ApiError {
    pub fn new(status_code: u16, message: String) -> Self {
        return ApiError {
            message,
            status_code,
        };
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return f.write_str(self.message.as_str());
    }
}

impl From<migration::DbErr> for ApiError {
    fn from(error: migration::DbErr) -> Self {
        return match error {
            error => ApiError::new(400, format!("Database error: {}", error.to_string())),
        };
    }
}

impl From<StoreError> for ApiError {
    fn from(error: StoreError) -> Self {
        match error {
            StoreError::HashError(error) => ApiError::new(500, format!("Hashing error: {}", error)),
            StoreError::DBError(error) => ApiError::new(500, format!("Diesel error: {}", error)),
            StoreError::PasswordNotMatch(error) => {
                ApiError::new(400, format!("Password error: {}", error))
            }
            StoreError::WrongPassword(error) => {
                ApiError::new(400, format!("Password error: {}", error))
            }
        }
    }
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
        let error = ApiError::new(self.status_code, self.message);
        let json = serde_json::to_string(&error).unwrap();

        Response::build_from(json.respond_to(request).unwrap())
            .status(Status::from_code(self.status_code).unwrap())
            .header(ContentType::JSON)
            .ok()
    }
}
