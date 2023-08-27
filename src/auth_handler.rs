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
    model::model::QueryOptions,
    response::UserListResponse,
    token::{self, TokenDetails},
    AppError,
};

use crate::{
    model::model::DB,
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

    body.id = Some(uuid_id);
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

// Login
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

    // let access_token_details = generate_token(
    //     user.id.unwrap(),
    //     data.env.access_token_max_age,
    //     data.env.access_token_private_key.to_owned(),
    // )?;
    // let refresh_token_details = generate_token(
    //     user.id,
    //     data.env.refresh_token_max_age,
    //     data.env.refresh_token_private_key.to_owned(),
    // )?;

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

fn _generate_token(
    user_id: uuid::Uuid,
    max_age: i64,
    private_key: String,
) -> Result<TokenDetails, (StatusCode, Json<serde_json::Value>)> {
    token::generate_jwt_token(user_id, max_age, private_key).map_err(|e| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("error generating token: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })
}
