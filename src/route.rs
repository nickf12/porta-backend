use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::{
        create_project_handler, delete_project_handler, edit_project_handler, get_project_handler,get_bounty_handler,
        porta_handler, projects_list_handler,bounty_list_handler, create_bounty_handler     , edit_bounty_handler, get_all_bounty_from_project_handler},
    model,
};

pub fn create_router() -> Router {
    let db = model::porta_db();

    Router::new()
        .route("/api/porta", get(porta_handler))
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
        .route(
            "/api/bounties",
            post(create_bounty_handler).get(bounty_list_handler),
        )
        .route(
            "/api/bounties/project/:id",
            get(get_all_bounty_from_project_handler),


        )
        .route(
            "/api/bounties/:id",
            get(get_bounty_handler)
                // )
                 .patch(edit_bounty_handler)
                 )
        //         // .delete(delete_bounty_handler),
        // )
        .with_state(db)
}
