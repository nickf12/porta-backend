use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;



#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Database {
    pub Projects :Vec<Project>,
    pub Bounties :Vec<Bounty>,
}

impl Database{ fn new ()->Database{
    Database {Projects: Vec::new(), Bounties: Vec::new()}
}}

pub type DB = Arc<Mutex<Database>>;


#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Project {
    pub id: Option<String>,
    pub title: String,
    pub content: String,
    pub createdAt: Option<DateTime<Utc>>,
    pub completed: Option<bool>,
    pub updatedAt: Option<DateTime<Utc>>,
}



#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Reward {
    amount: Option<u32>,
    denom: Option<String>
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Bounty {
    pub id: Option<String>,
    pub project_id: Option<String>,
    pub title: Option<String>,
    pub reward: Option<Reward >,
    pub completed : Option<bool>,
    pub createdAt: Option<DateTime<Utc>>,
    pub updatedAt: Option<DateTime<Utc>>,
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BountyRequest {
    pub id: Option<String>
}


// pub type DbBounty = Arc<Mutex<Vec<Bounty>>>;

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
pub struct UpdateProjectSchema {
    pub title: Option<String>,
    pub content: Option<String>,
    pub completed: Option<bool>,
}
