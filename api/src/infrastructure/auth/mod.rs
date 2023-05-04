mod jwt;
pub use jwt::*;

use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome, Request},
};

#[derive(Debug)]
pub enum AuthError {
    Missing,
    Unauthorized,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Claims {
    type Error = AuthError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("authorization");

        let Some(mut jwt) = auth_header else {
            return Outcome::Failure((Status::Unauthorized, AuthError::Missing));
        };

        if !jwt.contains("Bearer") {
            return Outcome::Failure((Status::Unauthorized, AuthError::Unauthorized));
        }

        jwt = jwt.split(" ").collect::<Vec<&str>>()[1];
        match jwt::decode_token(jwt) {
            Ok(claims) => Outcome::Success(claims),
            Err(error) => Outcome::Failure((Status::Unauthorized, AuthError::Unauthorized)),
        }
    }
}
