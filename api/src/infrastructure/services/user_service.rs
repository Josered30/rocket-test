use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::NaiveDateTime;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, QueryFilter, Set};

use crate::cores::errors::{ApiError, StoreError};

use crate::domain::models::user;
use crate::domain::models::user::Entity as User;
use crate::infrastructure::auth::create_token;

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
    pub fn hash_password(plain: &str) -> Result<String, StoreError> {
        Ok(hash(plain, DEFAULT_COST)?)
    }

    pub async fn create_user(
        db: &DbConn,
        email: String,
        password: String,
    ) -> Result<i32, ApiError> {
        let password = UserService::hash_password(&password)?;
        let result = user::ActiveModel {
            email: Set(email),
            password: Set(password),
            ..Default::default()
        }
        .save(db)
        .await?;

        return Ok(result.id.unwrap());
    }

    pub async fn sign_in(
        db: &DbConn,
        email: String,
        password: String,
    ) -> Result<(i32, String), ApiError> {
        let option_user = User::find()
            .filter(user::Column::Email.eq(email))
            .one(db)
            .await?;

        let Some(user) = option_user else {
            return Err(ApiError::new(400, "User not found".to_string()));
        };

        let Ok(valid) = verify(password, &user.password) else {
            return Err(ApiError::new(401, "Password validation error".to_string()));
        };

        if !valid {
            return Err(ApiError::new(401, "Password validation error".to_string()));
        }

        let jwt = create_token(user.email.as_str(), user.id)?;
        return Ok((user.id, jwt));
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
