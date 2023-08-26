use crate::log::log_request;

pub use self::error::{AppError, Result};

mod auth;
mod auth_handler;
mod ctx;
mod error;
mod handler;
mod log;
mod model;
mod mw;
mod response;
mod route;
mod token;

use std::net::SocketAddr;

use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method, Uri,
    },
    response::{IntoResponse, Response},
    Json,
};
use ctx::Ctx;
use once_cell::sync::Lazy;
use route::create_router;
use serde_json::json;
use tower_http::cors::CorsLayer;
use uuid::Uuid;

// TODO: Add secret key for JWT token
static _KEYS: Lazy<auth::_Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "Your secret here".to_owned());
    auth::_Keys::_new(secret.as_bytes())
});

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router().layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("🚀 Server started successfully");
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Axum special layer that take a response and return a response
async fn main_response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    let uuid = Uuid::new_v4();

    // -- Get the eventual response error.
    let service_error = res.extensions().get::<AppError>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    // -- If client error, build the new reponse.
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            });

            println!("    ->> client_error_body: {client_error_body}");

            // Build the new response from the client_error_body
            (*status_code, Json(client_error_body)).into_response()
        });

    // Build and log the server log line.
    let client_error = client_status_error.unzip().1;
    // TODO: Need to hander if log_request fail (but should not fail request)
    let _ = log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

    println!();
    error_response.unwrap_or(res)
}
