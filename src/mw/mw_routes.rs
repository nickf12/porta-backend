use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use tokio::sync::Mutex;

use crate::{
    auth_handler::user_list_handler,
    handler::{
        bounty_list_handler, create_bounty_handler, create_project_handler, delete_bounty_handler,
        delete_project_handler, edit_bounty_handler, edit_project_handler,
        get_all_bounty_from_project_handler, get_bounty_handler, get_project_handler,
        projects_list_handler,
    },
    model::model::Database,
};

use crate::model::model::DB;

pub fn mw_routes(db: DB) -> Router<Arc<Mutex<Database>>> {
    Router::new()
        .route(
            "/projects",
            post(create_project_handler).get(projects_list_handler),
        )
        .route(
            "/projects/:id",
            get(get_project_handler)
                .patch(edit_project_handler)
                .delete(delete_project_handler),
        )
        .route("/all/bounties", get(bounty_list_handler))
        .route(
            "/projects/:id/bounties",
            get(get_all_bounty_from_project_handler),
        )
        .route("/projects/:id/bounty", post(create_bounty_handler))
        .route(
            "/projects/:id/bounty/:id",
            get(get_bounty_handler)
                .patch(edit_bounty_handler)
                .delete(delete_bounty_handler),
        )
        .route("/users", get(user_list_handler))
        .with_state(db)
}
