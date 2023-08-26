use crate::auth::User;
use crate::model::{Bounty, Project};
use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: &'static str,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct ProjectData {
    pub project: Project,
}

#[derive(Serialize, Debug)]
pub struct SingleProjectResponse {
    pub status: String,
    pub data: ProjectData,
}

#[derive(Serialize, Debug)]
pub struct ProjectListResponse {
    pub status: String,
    pub results: usize,
    pub projects: Vec<Project>,
}

#[derive(Serialize, Debug)]
pub struct UserListResponse {
    pub status: String,
    pub results: usize,
    pub users: Vec<User>,
}

#[derive(Serialize, Debug)]
pub struct UserData {
    pub user: User,
}

#[derive(Serialize, Debug)]
pub struct SingleUserResponse {
    pub status: String,

    pub data: UserData,
}

#[derive(Serialize, Debug)]
pub struct BountyListResponse {
    pub status: String,
    pub results: usize,
    pub bounties: Vec<Bounty>,
}

#[derive(Serialize, Debug)]
pub struct SingleBountyResponse {
    pub status: String,
    pub data: BountyData,
}

#[derive(Serialize, Debug)]
pub struct BountyData {
    pub bounty: Bounty,
}
