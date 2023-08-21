use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub type Result<T> = core::result::Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    InvalidToken,
    WrongCredential,
    MissingCredential,
    TokenCreation,
    InternalServerError,
    UserDoesNotExist,
    UserAlreadyExits,
    AuthFailNoAuthTokenCookie,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "an internal server error occured",
            ),
            Self::AuthFailNoAuthTokenCookie => (StatusCode::UNAUTHORIZED, "No Auth Token Cookie"),
            Self::InvalidToken => (StatusCode::BAD_REQUEST, "invalid token"),
            Self::MissingCredential => (StatusCode::BAD_REQUEST, "missing credential"),
            Self::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "failed to create token"),
            Self::WrongCredential => (StatusCode::UNAUTHORIZED, "wrong credentials"),
            Self::UserDoesNotExist => (StatusCode::UNAUTHORIZED, "User does not exist"),
            Self::UserAlreadyExits => (StatusCode::BAD_REQUEST, "User already exists"),
        };
        (status, Json(json!({ "error": err_msg }))).into_response()
    }
}
