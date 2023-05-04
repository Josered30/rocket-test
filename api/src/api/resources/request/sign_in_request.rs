use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
}
