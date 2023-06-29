use crate::model::{Project,Bounty};
use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct ProjectData {
    pub project: Project,
}


#[derive(Serialize, Debug)]
pub struct BountyData {
    pub bounty: Bounty,
}

#[derive(Serialize, Debug)]
pub struct SingleProjectResponse {
    pub status: String,
    pub data: ProjectData,
}


#[derive(Serialize, Debug)]
pub struct SingleBountyResponse {
    pub status: String,
    pub data: BountyData,
}
#[derive(Serialize, Debug)]
pub struct ProjectListResponse {
    pub status: String,
    pub results: usize,
    pub projects: Vec<Project>,
}


#[derive(Serialize, Debug)]
pub struct BountyListResponse {
    pub status: String,
    pub results: usize,
    pub bounties: Vec<Bounty>,
}
