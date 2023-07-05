use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{
    model::{Project, QueryOptions, UpdateProjectSchema, DB,Bounty,  BountyRequest, Database, UpdateBountySchema},
    response::{ProjectData, ProjectListResponse, SingleProjectResponse, BountyListResponse , SingleBountyResponse, BountyData},
};

// Porta Handler
pub async fn porta_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Build Simple CRUD API in Rust using Axum";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

// Axum Route Function to Fetch All Records
pub async fn projects_list_handler(
    opts: Option<Query<QueryOptions>>,
    State(db): State<DB>,
) -> impl IntoResponse {
    let projects = db.lock().await;

    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    // It's not too expensive to clone the entire vector?
    let projects: Vec<Project> = projects.Projects
        .clone()
        .into_iter()
        .skip(offset)
        .take(limit)
        .collect();

    let json_response = ProjectListResponse {
        status: "success".to_string(),
        results: projects.len(),
        projects,
    };

    Json(json_response)
}

// Axum Route Function to Retrieve a Record -> /api/projects/:id
pub async fn get_project_handler(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let vec = db.lock().await;

    if let Some(project) = vec.Projects.iter().find(|project| project.id == Some(id.to_owned())) {
        let json_response = SingleProjectResponse {
            status: "success".to_string(),
            data: ProjectData {
                project: project.clone(),
            },
        };
        return Ok((StatusCode::OK, Json(json_response)));
    }

    let error_response = serde_json::json!({
        "status": "fail",
        "message": format!("Todo with ID: {} not found", id)
    });
    Err((StatusCode::NOT_FOUND, Json(error_response)))
}

// Axum Route Function to Add a Record
pub async fn create_project_handler(
    State(db): State<DB>,
    Json(mut body): Json<Project>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut vec = db.lock().await;

    if let Some(project) = vec.Projects.iter().find(|project| project.title == body.title) {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Project with title: '{}' already exists", project.title),
        });
        return Err((StatusCode::CONFLICT, Json(error_response)));
    }

    let uuid_id = Uuid::new_v4();
    let datetime = chrono::Utc::now();

    body.id = Some(uuid_id.to_string());
    body.completed = Some(false);
    body.createdAt = Some(datetime);
    body.updatedAt = Some(datetime);

    let project = body.to_owned();

    vec.Projects.push(body);

    let json_response = SingleProjectResponse {
        status: "success".to_string(),
        data: ProjectData { project },
    };

    Ok((StatusCode::CREATED, Json(json_response)))
}

// Axum Route Function to Edit a Record
pub async fn edit_project_handler(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
    Json(body): Json<UpdateProjectSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let mut vec = db.lock().await;

    if let Some(project) = vec.Projects
        .iter_mut()
        .find(|project| project.id == Some(id.clone()))
    {
        let datetime = chrono::Utc::now();
        let title = body
            .title
            .to_owned()
            .unwrap_or_else(|| project.title.to_owned());
        let content = body
            .content
            .to_owned()
            .unwrap_or_else(|| project.content.to_owned());
        let completed = body.completed.unwrap_or(project.completed.unwrap());
        let payload = Project {
            id: project.id.to_owned(),
            title: if !title.is_empty() {
                title
            } else {
                project.title.to_owned()
            },
            content: if !content.is_empty() {
                content
            } else {
                project.content.to_owned()
            },
            completed: Some(completed),
            createdAt: project.createdAt,
            updatedAt: Some(datetime),
        };
        *project = payload;

        let json_response = SingleProjectResponse {
            status: "success".to_string(),
            data: ProjectData {
                project: project.clone(),
            },
        };
        Ok((StatusCode::OK, Json(json_response)))
    } else {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Project with ID: {} not found", id)
        });

        Err((StatusCode::NOT_FOUND, Json(error_response)))
    }
}

// Axum Route Function to Delete a Record -> /api/projects/:id
pub async fn delete_project_handler(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let mut vec = db.lock().await;

    if let Some(pos) = vec.Projects
        .iter()
        .position(|project| project.id == Some(id.clone()))
    {
        vec.Projects.remove(pos);
        return Ok((StatusCode::NO_CONTENT, Json("")));
    }

    let error_response = serde_json::json!({
        "status": "fail",
        "message": format!("Project with ID: {} not found", id)
    });
    Err((StatusCode::NOT_FOUND, Json(error_response)))
}

pub async fn get_all_bounty_from_project_handler (
    Path(id): Path<Uuid>,
    State(db): State<DB>,

) -> impl IntoResponse{
    // add code here
let id= id.to_string();
   let vec = db.lock().await; 
    // let bounties :Vec<&Bounty>= vec.Bounties.iter().filter(|bounty| bounty.project_id == id).collect();
    let bounties :Vec<Bounty>= vec.Bounties.clone().into_iter().filter(|bounty| bounty.project_id == id).collect();

    let json_response = BountyListResponse {
        status: "success".to_string(),
        results: bounties.len(),
        bounties: bounties,
    };

    Json(json_response)
}

//Bounty Handler


pub async fn bounty_list_handler(
    opts: Option<Query<QueryOptions>>,
    State(db): State<DB>,
) -> impl IntoResponse {
    let bounties = db.lock().await;

    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let bounties: Vec<Bounty> = bounties.Bounties
        .clone()
        .into_iter()
        .skip(offset)
        .take(limit)
        .collect();

    let json_response = BountyListResponse {
        status: "success".to_string(),
        results: bounties.len(),
        bounties,
    };

    Json(json_response)
}

// // Axum Route Function to Retrieve a Record -> /api/projects/:id
pub async fn get_bounty_handler(
    id: Path<Uuid>,
    State(db): State<DB>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let vec = db.lock().await;

    if let Some(bounty) = vec.Bounties.iter().find(|bounty| bounty.id == Some(id.to_owned())) {
        let json_response = SingleBountyResponse {
            status: "success".to_string(),
            data: BountyData {
                bounty: bounty.clone(),
            },
        };
        return Ok((StatusCode::OK, Json(json_response)));
    }

    let error_response = serde_json::json!({
        "status": "fail",
        "message": format!("Bounty with ID: {} not found", id)
    });
    Err((StatusCode::NOT_FOUND, Json(error_response)))
}
//
// // Axum Route Function to Add a Record
pub async fn create_bounty_handler(
    State(db): State<DB>,
    Json(mut body): Json<Bounty>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut vec = db.lock().await;

    if let Some(bounty) = vec.Bounties.iter().find(|bounty| bounty.title == body.title) {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Bounty with title: '{}' already exists", bounty.title),
        });
        return Err((StatusCode::CONFLICT, Json(error_response)));
    }

    let uuid_id = Uuid::new_v4();
    let datetime = chrono::Utc::now();

    body.id = Some(uuid_id.to_string());
    body.completed = Some(false);
    body.createdAt = Some(datetime);
    body.updatedAt = Some(datetime);

    let bounty = body.to_owned();

    vec.Bounties.push(body);

    let json_response = SingleBountyResponse {
        status: "success".to_string(),
        data: BountyData { bounty },
    };

    Ok((StatusCode::CREATED, Json(json_response)))
}
//
// // Axum Route Function to Edit a Record
pub async fn edit_bounty_handler(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
    Json(body): Json<UpdateBountySchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let mut vec = db.lock().await;

    if let Some(bounty) = vec.Bounties
        .iter_mut()
        .find(|bounty| bounty.id == Some(id.clone()))
    {
        let datetime = chrono::Utc::now();
        let title = body
            .title
            .to_owned()
            .unwrap_or_else(|| bounty.title.to_owned());
        let reward = body
            .reward
            .to_owned()
            .unwrap_or_else(|| bounty.reward.to_owned());
        let completed = body.completed.unwrap_or(bounty.completed.unwrap());
        let payload = Bounty {
            id: bounty.id.to_owned(),
            project_id: bounty.project_id.to_owned(),
            title: if !title.is_empty() {
                title
            } else {
                bounty.title.to_owned()
            },
            reward: bounty.reward.to_owned(),
            completed: Some(completed),
            createdAt: bounty.createdAt,
            updatedAt: Some(datetime),
        };
        *bounty = payload;

        let json_response = SingleBountyResponse {
            status: "success".to_string(),
            data: BountyData {
                bounty: bounty.clone(),
            },
        };
        Ok((StatusCode::OK, Json(json_response)))
    } else {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Project with ID: {} not found", id)
        });

        Err((StatusCode::NOT_FOUND, Json(error_response)))
    }
}
//
// // Axum Route Function to Delete a Record -> /api/projects/:id
pub async fn delete_bounty_handler(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let mut vec = db.lock().await;

    if let Some(pos) = vec.Bounties
        .iter()
        .position(|bounty| bounty.id == Some(id.clone()))
    {
        vec.Bounties.remove(pos);
        return Ok((StatusCode::NO_CONTENT, Json("")));
    }

    let error_response = serde_json::json!({
        "status": "fail",
        "message": format!("Bounty with ID: {} not found", id)
    });
    Err((StatusCode::NOT_FOUND, Json(error_response)))
}
