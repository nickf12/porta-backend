use axum::{
    middleware::{self, from_fn},
    routing::{get, post},
    Router,
};
use tower_cookies::CookieManagerLayer;

use crate::{
    auth_handler::{api_login, create_user_handler},
    handler::porta_handler,
    main_response_mapper, model,
    mw::{mw_auth::mw_require_auth, mw_routes::mw_routes},
};

pub fn create_router() -> Router {
    // TODO: Add database backup and restoring
    let db = model::model::porta_db();

    let mw_routes = mw_routes(db.clone()).route_layer(from_fn(mw_require_auth));

    Router::new()
        .route("/porta", get(porta_handler))
        .route("/api/login", post(api_login))
        .route("/api/user", post(create_user_handler))
        .nest("/api", mw_routes)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .with_state(db)
}
