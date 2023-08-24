use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde_json::json;
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

use crate::{
    auth::{User, AUTH_TOKEN},
    model::QueryOptions,
    response::UserListResponse,
    AppError,
};

use crate::{
    model::DB,
    response::{SingleUserResponse, UserData},
};
// Axum Route Function to Add a Record
pub async fn create_user_handler(
    State(db): State<DB>,
    Json(mut body): Json<User>,
) -> Result<impl IntoResponse, AppError> {
    println!("->> {:<12} - api_create_user", "HANDLER");

    let mut vec = db.lock().await;

    if vec.users.iter().any(|user| user.address == body.address) {
        return Err(AppError::UserAlreadyExits);
    }

    let uuid_id = Uuid::new_v4();
    let _datetime = chrono::Utc::now();

    body.id = Some(uuid_id.to_string());

    let user: User = body.to_owned();

    vec.users.push(body);

    let json_response = SingleUserResponse {
        status: "success".to_string(),
        data: UserData { user },
    };

    Ok((StatusCode::CREATED, Json(json_response)))
}

//  Axum Route to get All users handler
pub async fn user_list_handler(
    opts: Option<Query<QueryOptions>>,
    State(db): State<DB>,
) -> impl IntoResponse {
    println!("->> {:<12} - api_user_list", "HANDLER");

    let db = db.lock().await;

    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let users: Vec<User> = db
        .users
        .clone()
        .into_iter()
        .skip(offset)
        .take(limit)
        .collect();

    let json_response = UserListResponse {
        status: "success".to_string(),
        results: users.len(),
        users,
    };

    Json(json_response)
}

pub async fn api_login(
    cookies: Cookies,
    State(db): State<DB>,
    Json(body): Json<User>,
) -> Result<impl IntoResponse, AppError> {
    println!("->> {:<12} - api_login", "HANDLER");
    let vec = db.lock().await;

    if !vec.users.iter().any(|user| user.address == body.address) {
        return Err(AppError::UserDoesNotExist);
    }

    if !vec
        .users
        .iter()
        .any(|user| (user.address == body.address && user.password == body.password))
    {
        return Err(AppError::WrongCredential);
    }

    let user = vec
        .users
        .iter()
        .find(|user| (user.address == body.address && user.password == body.password))
        .unwrap();
    // FIXME: implement real auth-token generation/signature.
    cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));

    // Create the success body
    let body = Json(json!({
        "result": {
            "success":true,
            "user": user
        }

    }));
    Ok(body)
}
