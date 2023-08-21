use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use tokio::sync::Mutex;

use crate::{
    handler::{
        create_project_handler, delete_project_handler, edit_project_handler, get_project_handler,
        projects_list_handler,
    },
    model::Database,
};

use crate::model::DB;

pub fn mw_routes(db: DB) -> Router<Arc<Mutex<Database>>> {
    Router::new()
        .route(
            "/api/projects",
            post(create_project_handler).get(projects_list_handler),
        )
        .route(
            "/api/projects/:id",
            get(get_project_handler)
                .patch(edit_project_handler)
                .delete(delete_project_handler),
        )
        .with_state(db)
}
