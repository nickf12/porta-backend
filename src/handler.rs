use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use uuid::Uuid;

use crate::{
    model::model::{Bounty, Project, QueryOptions, UpdateBountySchema, UpdateProject, DB},
    response::{
        BountyData, BountyListResponse, ProjectData, ProjectListResponse, SingleBountyResponse,
        SingleProjectResponse,
    },
};

// Porta Handler
pub async fn porta_handler() -> impl IntoResponse {
    println!("->> {:<12} - api_porta", "HANDLER");

    const MESSAGE: &str = "Porta backend in Rust using Axum";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

// Axum Route Function to Fetch All Project Records
pub async fn projects_list_handler(
    opts: Option<Query<QueryOptions>>,
    State(db): State<DB>,
) -> impl IntoResponse {
    println!("->> {:<12} - api_project_list", "HANDLER");
    let projects = db.lock().await;

    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let projects: Vec<Project> = projects
        .projects
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
    Path(id): Path<String>,
    State(db): State<DB>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    println!("->> {:<12} - api_get_project", "HANDLER");

    let id = id.to_string();
    let vec = db.lock().await;

    if let Some(project) = vec.projects.iter().find(|project| project.project_id == id) {
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
        "message": format!("Project with ID: {} not found", id)
    });
    Err((StatusCode::NOT_FOUND, Json(error_response)))
}

// Axum Route Function to Add a Record
pub async fn create_project_handler(
    State(db): State<DB>,
    Json(mut body): Json<Project>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    println!("->> {:<12} - api_create_project", "HANDLER");

    let mut vec = db.lock().await;

    if let Some(project) = vec
        .projects
        .iter()
        .find(|project| project.project_id == body.project_id)
    {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Project with ID: '{}' already exists", project.project_id),
        });
        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }

    let uuid_id = Uuid::new_v4();
    let datetime = chrono::Utc::now();

    body.id = Some(uuid_id);
    body.completed = Some(false);
    body.createdAt = Some(datetime);
    body.updatedAt = Some(datetime);

    let project = body.to_owned();

    vec.projects.push(body);

    let json_response = SingleProjectResponse {
        status: "success".to_string(),
        data: ProjectData { project },
    };

    Ok((StatusCode::CREATED, Json(json_response)))
}

// Axum Route Function to Edit a Record
pub async fn edit_project_handler(
    Path(id): Path<String>,
    State(db): State<DB>,
    Json(body): Json<UpdateProject>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    println!("->> {:<12} - api_edit_project", "HANDLER");
    print!("ID -> {}", id.to_string());
    let id = id.to_string();
    let mut vec = db.lock().await;

    if let Some(project) = vec
        .projects
        .iter_mut()
        .find(|project| project.project_id == id.clone())
    {
        let datetime = chrono::Utc::now();
        let project_id = body
            .project_id
            .to_owned()
            .unwrap_or_else(|| project.project_id.to_owned());
        let project_name = body
            .project_id
            .to_owned()
            .unwrap_or_else(|| project.project_name.to_owned());
        let project_deliverables = body
            .project_deliverables
            .to_owned()
            .unwrap_or_else(|| project.project_deliverables.to_owned());
        let project_description = body
            .project_lead_address
            .to_owned()
            .unwrap_or_else(|| project.project_description.to_owned());
        let project_lead_address = body
            .project_id
            .to_owned()
            .unwrap_or_else(|| project.project_lead_address.to_owned());
        let project_budget = body
            .project_budget
            .to_owned()
            .unwrap_or_else(|| project.project_budget.to_owned());
        let project_lead_reward = body
            .project_lead_reward
            .to_owned()
            .unwrap_or_else(|| project.project_lead_reward.to_owned());
        let completed = body.completed.unwrap_or(project.completed.unwrap());
        let project_denom = body
            .project_denom
            .to_owned()
            .unwrap_or_else(|| project.project_denom.to_owned());
        let project_type = body
            .project_type
            .to_owned()
            .unwrap_or_else(|| project.project_type.to_owned());

        let payload = Project {
            id: project.id.to_owned(),
            project_id,
            project_budget,
            project_deliverables,
            project_denom,
            project_description,
            project_lead_address,
            project_lead_reward,
            project_name,
            project_type,

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
    Path(id): Path<String>,
    State(db): State<DB>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    println!("->> {:<12} - api_delete_project", "HANDLER");

    let id = id.to_string();
    let mut vec = db.lock().await;

    if let Some(pos) = vec
        .projects
        .iter()
        .position(|project| project.project_id == id.clone())
    {
        vec.projects.remove(pos);
        return Ok((StatusCode::NO_CONTENT, Json("")));
    }

    let error_response = serde_json::json!({
        "status": "fail",
        "message": format!("Project with ID: {} not found", id)
    });
    Err((StatusCode::NOT_FOUND, Json(error_response)))
}

pub async fn get_all_bounty_from_project_handler(
    Path(id): Path<String>,
    State(db): State<DB>,
) -> impl IntoResponse {
    println!("->> {:<12} - get_all_project_bounties", "HANDLER");

    // add code here
    let id = id.to_string();
    let vec = db.lock().await;
    // let bounties :Vec<&Bounty>= vec.Bounties.iter().filter(|bounty| bounty.project_id == id).collect();
    let bounties: Vec<Bounty> = vec
        .bounties
        .clone()
        .into_iter()
        .filter(|bounty| bounty.project_id == id)
        .collect();

    let json_response = BountyListResponse {
        status: "success".to_string(),
        results: bounties.len(),
        bounties,
    };

    Json(json_response)
}

//Bounty Handler

pub async fn bounty_list_handler(
    opts: Option<Query<QueryOptions>>,
    State(db): State<DB>,
) -> impl IntoResponse {
    println!("->> {:<12} - api_get_bounty_list", "HANDLER");

    let bounties = db.lock().await;

    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let bounties: Vec<Bounty> = bounties
        .bounties
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
    println!("->> {:<12} - api_get_bounty", "HANDLER");

    if let Some(bounty) = vec
        .bounties
        .iter()
        .find(|bounty| bounty.bounty_id == id.to_owned())
    {
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
    Path(id): Path<Uuid>,
    State(db): State<DB>,
    Json(mut body): Json<Bounty>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut vec = db.lock().await;
    println!("->> {:<12} - api_create_bounty", "HANDLER");

    if let Some(bounty) = vec
        .bounties
        .iter()
        .find(|bounty| bounty.title == body.title)
    {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Bounty with title: '{}' already exists", bounty.title),
        });
        return Err((StatusCode::CONFLICT, Json(error_response)));
    }

    let uuid_id = Uuid::new_v4();
    let datetime = chrono::Utc::now();

    body.id = Some(uuid_id);
    body.project_id = id.to_string();
    body.completed = Some(false);
    body.createdAt = Some(datetime);
    body.updatedAt = Some(datetime);

    let bounty = body.to_owned();

    vec.bounties.push(body);

    let json_response = SingleBountyResponse {
        status: "success".to_string(),
        data: BountyData { bounty },
    };

    Ok((StatusCode::CREATED, Json(json_response)))
}
//
// // Axum Route Function to Edit a Record
pub async fn edit_bounty_handler(
    Path((id, bounty_id)): Path<(String, String)>,
    State(db): State<DB>,
    Json(body): Json<UpdateBountySchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    println!("->> {:<12} - api_edit_bounty", "HANDLER");

    let id = id.to_string();
    let mut vec = db.lock().await;

    if let Some(bounty) = vec
        .bounties
        .iter_mut()
        .find(|bounty| bounty.bounty_id == bounty_id.clone())
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
            bounty_id: body
                .bounty_id
                .to_owned()
                .unwrap_or_else(|| bounty.bounty_id.to_owned()),
            project_id: body
                .project_id
                .to_owned()
                .unwrap_or_else(|| bounty.project_id.to_owned()),
            title,
            reward,
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
    Path((_project_id, bounty_id)): Path<(String, String)>,
    State(db): State<DB>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut vec = db.lock().await;

    if let Some(pos) = vec
        .bounties
        .iter()
        .position(|bounty| bounty.bounty_id == bounty_id.clone())
    {
        vec.bounties.remove(pos);
        return Ok((StatusCode::NO_CONTENT, Json("")));
    }

    let error_response = serde_json::json!({
        "status": "fail",
        "message": format!("Bounty with ID: {} not found", bounty_id)
    });
    Err((StatusCode::NOT_FOUND, Json(error_response)))
}
