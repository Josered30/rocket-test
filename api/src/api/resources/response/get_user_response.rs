use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct GetUserResponse {
    pub id: i32,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl GetUserResponse {
    pub fn new(
        id: i32,
        email: String,
        created_at: NaiveDateTime,
        updated_at: Option<NaiveDateTime>,
    ) -> Self {
        Self {
            id,
            email,
            created_at,
            updated_at,
        }
    }
}
