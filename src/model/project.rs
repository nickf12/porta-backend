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

    pub async fn first_by_project_id<E>(
        _ctx: &Ctx,
        mm: &ModelManager,
        project_id: &str,
    ) -> Result<Option<E>>
    where
        E: ProjectBy,
    {
        let db = mm.db();
        let project = sqlb::select()
            .table(self::DbBmc::TABLE)
            .and_where("project_id", "=", project_id)
            .fetch_optional::<_, E>(db)
            .await?;
        Ok(project)
    }

    pub async fn update_project_lead_addr(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        project_lead_addr: &str,
    ) -> Result<()> {
        let db = mm.db();

        let project: Project = Self::get(ctx, mm, id).await?;

        sqlb::update()
            .table(Self::TABLE)
            .and_where("project_lead_address", "=", id)
            .data(vec![(
                "project_lead_address",
                project_lead_addr.to_string(),
            )
                .into()])
            .exec(db)
            .await?;
        Ok(())
    }
    // TODO: Implement Update for almost all project's fields

    // TODO: Implement delete project

    // TODO: Implement other getters
}
