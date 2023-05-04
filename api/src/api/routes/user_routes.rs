use rocket::fairing::AdHoc;
use sea_orm_rocket::Connection;

use crate::{
    api::resources::{request::*, response::*},
    cores::errors::ApiError,
    database::Db,
    infrastructure::{auth::Claims, services::user_service::UserService},
};

use rocket::serde::json::Json;

#[post("/register", format = "json", data = "<create_user_request>")]
async fn register(
    conn: Connection<'_, Db>,
    create_user_request: Json<RegisterUserRequest>,
) -> Result<Json<RegisterUserResponse>, ApiError> {
    let db = conn.into_inner();
    let create_user = create_user_request.into_inner();
    let id = UserService::create_user(db, create_user.email, create_user.password).await?;

    return Ok(Json(RegisterUserResponse { id }));
}

#[post("/sign-in", format = "json", data = "<sign_in_request>")]
async fn sign_in(
    conn: Connection<'_, Db>,
    sign_in_request: Json<SignInRequest>,
) -> Result<Json<SignInResponse>, ApiError> {
    let db = conn.into_inner();
    let sign_in = sign_in_request.into_inner();

    let (id, jwt) = UserService::sign_in(db, sign_in.email, sign_in.password).await?;
    return Ok(Json(SignInResponse { id, jwt }));
}

#[get("/", format = "json")]
async fn list(conn: Connection<'_, Db>, claims: Claims) -> Json<ListUserResponse> {
    let db = conn.into_inner();
    let result = UserService::list_users(db).await;

    let users = match result {
        Ok(value) => value,
        _ => Vec::new(),
    };

    let users_response = users
        .into_iter()
        .map(|user_info| {
            GetUserResponse::new(
                user_info.id,
                user_info.email,
                user_info.created_at,
                user_info.updated_at,
            )
        })
        .collect::<Vec<GetUserResponse>>();
    return Json(ListUserResponse::new(users_response));
}

pub fn user_stage() -> AdHoc {
    AdHoc::on_ignite("User routes stage", |rocket| async {
        rocket.mount("/api/users", routes![register, list, sign_in])
    })
}
