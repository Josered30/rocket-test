use serde::{Deserialize, Serialize};

use super::get_user_response::GetUserResponse;

#[derive(Serialize)]
pub struct ListUserResponse {
    pub users: Vec<GetUserResponse>,
}

impl ListUserResponse {
    pub fn new(users: Vec<GetUserResponse>) -> Self {
        Self { users }
    }
}
