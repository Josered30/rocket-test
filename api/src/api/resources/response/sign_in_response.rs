use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SignInResponse {
    pub id: i32,
    pub jwt: String,
}
