use chrono::NaiveDateTime;
use rocket::{fairing::AdHoc};
use sea_orm_rocket::Connection;
use serde::{Deserialize, Serialize};

use crate::{database::Db, errors::ApiError, services::user_service::UserService};

use rocket::serde::json::Json;

#[derive(Serialize)]
struct UserRegisterResponse {
    id: i32,
}

#[derive(Serialize)]
struct UserResponse {
    pub id: i32,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl UserResponse {
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

#[derive(Serialize)]
struct ListUserResponse {
    users: Vec<UserResponse>,
}

#[derive(Deserialize, Serialize)]
struct CreateUserRequest {
    email: String,
    password: String,
}

impl ListUserResponse {
    pub fn new(users: Vec<UserResponse>) -> Self {
        Self { users }
    }
}

#[post("/register", format = "json", data = "<create_user_request>")]
async fn register(
    conn: Connection<'_, Db>,
    create_user_request: Json<CreateUserRequest>,
) -> Result<Json<UserRegisterResponse>, ApiError> {
    let db = conn.into_inner();

    let create_user = create_user_request.into_inner();
    let id = UserService::create_user(db, create_user.email, create_user.password).await?;

    return Ok(Json(UserRegisterResponse { id }));
}

#[get("/", format = "json")]
async fn list(conn: Connection<'_, Db>) -> Json<ListUserResponse> {
    let db = conn.into_inner();
    let result = UserService::list_users(db).await;

    let users = match result {
        Ok(value) => value,
        _ => Vec::new(),
    };

    let users_response = users
        .into_iter()
        .map(|user_info| {
            UserResponse::new(
                user_info.id,
                user_info.email,
                user_info.created_at,
                user_info.updated_at,
            )
        })
        .collect::<Vec<UserResponse>>();
    return Json(ListUserResponse::new(users_response));
}

pub fn user_stage() -> AdHoc {
    AdHoc::on_ignite("User routes stage", |rocket| async {
        rocket.mount("/api/users", routes![register, list])
    })
}
