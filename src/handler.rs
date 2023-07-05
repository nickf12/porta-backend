use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use tracing_subscriber::fmt::writer::OrElse;
use uuid::Uuid;

use crate::{
    model::{
        Bounty, BountyRequest, Database, Project, QueryOptions, UpdateBountySchema, UpdateProject,
        UpdateProjectSchema, User, DB,
    },
    response::{
        ProjectData, ProjectListResponse, SingleProjectResponse, SingleUserResponse, UserData,
        UserListResponse, BountyListResponse , SingleBountyResponse, BountyData
    },
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

// Axum Route Function to Fetch All Project Records
pub async fn projects_list_handler(
    opts: Option<Query<QueryOptions>>,
    State(db): State<DB>,
) -> impl IntoResponse {
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
    Path(id): Path<Uuid>,
    State(db): State<DB>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let vec = db.lock().await;

    if let Some(project) = vec
        .projects
        .iter()
        .find(|project| project.id == Some(id.to_owned()))
    {
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

    if let Some(project) = vec
        .projects
        .iter()
        .find(|project| project.project_id == body.project_id)
    {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Project with ID: '{}' already exists", project.project_id),
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

    vec.projects.push(body);

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
    Json(body): Json<UpdateProject>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let mut vec = db.lock().await;

    if let Some(project) = vec
        .projects
        .iter_mut()
        .find(|project| project.id == Some(id.clone()))
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
    Path(id): Path<Uuid>,
    State(db): State<DB>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let mut vec = db.lock().await;

    if let Some(pos) = vec
        .projects
        .iter()
        .position(|project| project.id == Some(id.clone()))
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

//  Axum Route to get All users handler
pub async fn user_list_handler(
    opts: Option<Query<QueryOptions>>,
    State(db): State<DB>,
) -> impl IntoResponse {
    let users = db.lock().await;

    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let users: Vec<User> = users
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

// Axum Route Function to Add a Record
pub async fn _create_user_handler(
    State(db): State<DB>,
    Json(mut body): Json<User>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut vec = db.lock().await;

    if let Some(user) = vec.users.iter().find(|user| user.address == body.address) {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Project with title: '{}' already exists", user.address),
        });
        return Err((StatusCode::CONFLICT, Json(error_response)));
    }

    let uuid_id = Uuid::new_v4();
    let _datetime = chrono::Utc::now();

    body.id = Some(uuid_id.to_string());

    let user: User = body.to_owned();

    vec.users.push(body);

    let json_response = SingleUserResponse {
        status: "success".to_string(),
        data: UserData { user },
    };

    Ok((StatusCode::CREATED, Json(json_response)))
}
pub async fn get_all_bounty_from_project_handler(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
) -> impl IntoResponse {
    // add code here
    let id = id.to_string();
    let vec = db.lock().await;
    // let bounties :Vec<&Bounty>= vec.Bounties.iter().filter(|bounty| bounty.project_id == id).collect();
    let bounties: Vec<Bounty> = vec
        .Bounties
        .clone()
        .into_iter()
        .filter(|bounty| bounty.project_id == id)
        .collect();

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

    let bounties: Vec<Bounty> = bounties
        .Bounties
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

    if let Some(bounty) = vec
        .Bounties
        .iter()
        .find(|bounty| bounty.id == Some(id.to_owned()))
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
    State(db): State<DB>,
    Json(mut body): Json<Bounty>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut vec = db.lock().await;

    if let Some(bounty) = vec
        .Bounties
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

    if let Some(bounty) = vec
        .Bounties
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

    if let Some(pos) = vec
        .Bounties
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
