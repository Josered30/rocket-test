use bcrypt::{hash, DEFAULT_COST};
use chrono::NaiveDateTime;
use sea_orm::{ActiveModelTrait, DbConn, EntityTrait, Set};
use serde::{Deserialize, Serialize};

use crate::errors::{ApiError, StoreError};

use crate::models::user;
use crate::models::user::Entity as User;

pub struct UserInfo {
    pub id: i32,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl UserInfo {
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

pub struct UserService;

impl UserService {
    pub fn hash_password(plain: String) -> Result<String, StoreError> {
        Ok(hash(plain, DEFAULT_COST)?)
    }

    pub async fn create_user(
        db: &DbConn,
        email: String,
        password: String,
    ) -> Result<i32, ApiError> {
        let password = UserService::hash_password(password)?;
        let result = user::ActiveModel {
            email: Set(email),
            password: Set(password),
            ..Default::default()
        }
        .save(db)
        .await?;

        return Ok(result.id.unwrap());
    }

    pub async fn list_users(db: &DbConn) -> Result<Vec<UserInfo>, ApiError> {
        let result = User::find().all(db).await?;

        let users: Vec<UserInfo> = result
            .into_iter()
            .map(|model| UserInfo::new(model.id, model.email, model.created_at, model.updated_at))
            .collect::<Vec<UserInfo>>();

        return Ok(users);
    }
}
