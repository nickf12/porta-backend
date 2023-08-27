use crate::model::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlb::HasFields;
use sqlx::{postgres::PgRow, FromRow};

use crate::ctx::Ctx;

use super::{
    base::{self, DbBmc},
    ModelManager,
};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Project {
    pub id: i64,
    pub project_id: String,
    pub project_name: String,
    pub project_lead_address: String,
    pub project_budget: String,
    pub project_denom: String,
    pub project_type: String,
    pub project_description: String,
}

#[derive(Deserialize)]
pub struct ProjectForCreate {
    pub id: i64,
    pub project_id: String,
    pub project_name: String,
    pub project_lead_address: String,
    pub project_budget: String,
    pub project_denom: String,
    pub project_type: String,
    pub project_description: String,
}

pub trait ProjectBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl ProjectBy for Project {}

pub struct ProjectBmc;

impl DbBmc for ProjectBmc {
    const TABLE: &'static str = "project";
}

impl ProjectBmc {
    pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
    where
        E: ProjectBy,
    {
        base::get::<Self, _>(ctx, mm, id).await
    }
}
