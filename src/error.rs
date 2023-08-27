// use axum::{http::StatusCode, response::IntoResponse, response::Response};
// use serde::Serialize;

// pub type Result<T> = core::result::Result<T, AppError>;

// #[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
// #[serde(tag = "type", content = "data")]
// pub enum AppError {
//     InvalidToken,
//     WrongCredential,
//     MissingCredential,
//     TokenCreation,
//     InternalServerError,
//     UserDoesNotExist,
//     UserAlreadyExits,
//     AuthFailNoAuthTokenCookie,
//     AuthFailTokenWrongFormat,
//     AuthFailCtxNotInRequestExt,
//     LoginFail,
// }

// impl IntoResponse for AppError {
//     fn into_response(self) -> Response {
//         println!("->> {:<12} - {self:?}", "INTO_RES");

//         // Create a placeholder Axum reponse.
//         let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

//         // Insert the Error into the reponse.
//         response.extensions_mut().insert(self);

//         response
//     }
// }

// impl AppError {
//     pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
//         #[allow(unreachable_patterns)]
//         match self {
//             // -- Login
//             Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

//             // -- Auth.
//             Self::AuthFailNoAuthTokenCookie
//             | Self::AuthFailTokenWrongFormat
//             | Self::AuthFailCtxNotInRequestExt => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

//             // -- Model.
//             // Add Project / Bounty errors
//             // -- Falback.
//             _ => (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 ClientError::SERVICE_ERROR,
//             ),
//         }
//     }
// }

// #[derive(Debug, strum_macros::AsRefStr)]
// #[allow(non_camel_case_types)]
// pub enum ClientError {
//     LOGIN_FAIL,
//     NO_AUTH,
//     INVALID_PARAMS,
//     SERVICE_ERROR,
// }

use crate::model;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    // -- Config
    ConfigMissingEnv(&'static str),
    ConfigWrongFormat(&'static str),

    // -- Modules
    Model(model::Error),
}

// region:    --- Froms
impl From<model::Error> for Error {
    fn from(val: model::Error) -> Self {
        Self::Model(val)
    }
}
// endregion: --- Froms

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
