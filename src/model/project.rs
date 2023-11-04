use crate::model::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlb::{Fields, HasFields};
use sqlx::{postgres::PgRow, FromRow};

use crate::ctx::Ctx;

use super::{
    base::{self, DbBmc},
    ModelManager,
};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, Fields, FromRow)]
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

#[derive(Deserialize, Fields)]
pub struct ProjectForCreate {
    pub project_id: String,
    pub project_name: String,
    pub project_lead_address: String,
    pub project_budget: String,
    pub project_denom: String,
    pub project_type: String,
    pub project_description: String,
}
#[derive(Deserialize, Fields)]
pub struct ProjectForUpdate {
    pub id: i64,
    pub project_id: Option<String>,
    pub project_name: Option<String>,
    pub project_lead_address: Option<String>,
    pub project_budget: Option<String>,
    pub project_denom: Option<String>,
    pub project_type: Option<String>,
    pub project_description: Option<String>,
}

pub trait ProjectBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl ProjectBy for Project {}

pub struct ProjectBmc;

impl DbBmc for ProjectBmc {
    const TABLE: &'static str = "project";
}

impl ProjectBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, project_c: ProjectForCreate) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, project_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Project> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Project>> {
        base::list::<Self, _>(ctx, mm).await
    }
    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        bounty_u: ProjectForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, bounty_u).await
    }
    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }

    // TODO: Implement Update for almost all project's fields

    // TODO: Implement delete project

    // TODO: Implement other getters
}
