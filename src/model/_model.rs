use crate::auth::User;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Project {
    pub id: Option<Uuid>,
    pub project_id: String,
    pub project_name: String,
    pub project_lead_address: String,
    pub project_lead_reward: i32,
    pub project_budget: f64,
    pub project_denom: String,
    pub project_type: String,
    // TODO: pub project_bounties: Vec<Bounty>, Enable this field when bounty module is ready
    pub project_description: String,
    pub project_deliverables: String,
    pub completed: Option<bool>,
    pub createdAt: Option<DateTime<Utc>>,
    pub updatedAt: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Reward {
    amount: u32,
    denom: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Bounty {
    pub id: Option<Uuid>,
    pub bounty_id: String,
    pub project_id: String,
    pub title: String,
    pub reward: Reward,
    pub completed: Option<bool>,
    pub createdAt: Option<DateTime<Utc>>,
    pub updatedAt: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BountyRequest {
    pub id: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Database {
    pub projects: Vec<Project>,
    pub users: Vec<User>,
    pub bounties: Vec<Bounty>,
}
impl Database {
    fn new() -> Database {
        Database {
            projects: Vec::new(),
            users: Vec::new(),
            bounties: Vec::new(),
        }
    }
}
pub type DB = Arc<Mutex<Database>>;

pub fn porta_db() -> DB {
    Arc::new(Mutex::new(Database::new()))
}

#[derive(Debug, Deserialize, Default)]
pub struct QueryOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateProject {
    pub project_id: Option<String>,
    pub project_name: Option<String>,
    pub project_lead_address: Option<String>,
    pub project_lead_reward: Option<i32>,
    pub project_budget: Option<f64>,
    pub project_denom: Option<String>,
    pub project_type: Option<String>,
    // pub project_bounties: Vec<Bounty>, Enable this field when bounty module is ready
    pub project_description: Option<String>,
    pub project_deliverables: Option<String>,
    pub completed: Option<bool>,
    pub createdAt: Option<DateTime<Utc>>,
    pub updatedAt: Option<DateTime<Utc>>,
}
#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateBountySchema {
    pub bounty_id: Option<String>,
    pub project_id: Option<String>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub completed: Option<bool>,
    pub reward: Option<Reward>,
}
