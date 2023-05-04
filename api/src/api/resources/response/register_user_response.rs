use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct RegisterUserResponse {
    pub id: i32,
}
