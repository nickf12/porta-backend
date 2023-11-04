use crate::ctx::Ctx;
use crate::model::bounty::{Bounty, BountyBmc, BountyForCreate, BountyForUpdate};
use crate::model::project::{Project, ProjectBmc, ProjectForCreate, ProjectForUpdate};
use crate::model::ModelManager;
use crate::web::rpc::{ParamsForCreate, ParamsForUpdate, ParamsIded};
use crate::web::Result;
// -- TODO: Add permission layer for create, update and delete routes
pub async fn create_bounty(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<BountyForCreate>,
) -> Result<Bounty> {
    let ParamsForCreate { data } = params;

    let id = BountyBmc::create(&ctx, &mm, data).await?;
    let task = BountyBmc::get(&ctx, &mm, id).await?;

    Ok(task)
}

pub async fn list_bounty(ctx: Ctx, mm: ModelManager) -> Result<Vec<Bounty>> {
    let tasks = BountyBmc::list(&ctx, &mm).await?;

    Ok(tasks)
}

pub async fn update_bounty(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<BountyForUpdate>,
) -> Result<Bounty> {
    let ParamsForUpdate { id, data } = params;

    BountyBmc::update(&ctx, &mm, id, data).await?;

    let task = BountyBmc::get(&ctx, &mm, id).await?;

    Ok(task)
}

pub async fn delete_bounty(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<Bounty> {
    let ParamsIded { id } = params;

    let task = BountyBmc::get(&ctx, &mm, id).await?;
    BountyBmc::delete(&ctx, &mm, id).await?;

    Ok(task)
}

pub async fn create_project(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<ProjectForCreate>,
) -> Result<Project> {
    let ParamsForCreate { data } = params;

    let id = ProjectBmc::create(&ctx, &mm, data).await?;
    let task = ProjectBmc::get(&ctx, &mm, id).await?;

    Ok(task)
}

pub async fn list_project(ctx: Ctx, mm: ModelManager) -> Result<Vec<Project>> {
    let tasks = ProjectBmc::list(&ctx, &mm).await?;

    Ok(tasks)
}

pub async fn update_project(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<ProjectForUpdate>,
) -> Result<Project> {
    let ParamsForUpdate { id, data } = params;

    ProjectBmc::update(&ctx, &mm, id, data).await?;

    let task = ProjectBmc::get(&ctx, &mm, id).await?;

    Ok(task)
}

pub async fn delete_project(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<Project> {
    let ParamsIded { id } = params;

    let task = ProjectBmc::get(&ctx, &mm, id).await?;
    ProjectBmc::delete(&ctx, &mm, id).await?;

    Ok(task)
}
