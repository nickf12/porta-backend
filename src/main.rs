use std::net::SocketAddr;

use axum::{response::IntoResponse, routing::get, Json, Router};

mod handler;
mod model;
mod response;
mod route;

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use route::create_router;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router().layer(cors);
    
    let addr= SocketAddr::from(([0,0,0,1],8000));

    println!(" Server started successfully");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


