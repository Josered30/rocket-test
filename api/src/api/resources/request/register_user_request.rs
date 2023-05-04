use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct RegisterUserRequest {
    pub email: String,
    pub password: String,
}
