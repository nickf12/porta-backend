pub use self::error::{AppError, Result};

mod auth;
mod auth_handler;
mod error;
mod handler;
mod model;
mod mw;
mod response;
mod route;

use std::net::SocketAddr;

use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    response::Response,
};
use once_cell::sync::Lazy;
use route::create_router;
use tower_http::cors::CorsLayer;

// secret key for JWT token
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
async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
}
