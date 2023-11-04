use crate::ctx::Ctx;
use crate::model::bounty::{Bounty, BountyBmc, BountyForCreate, BountyForUpdate};
use crate::model::ModelManager;
use crate::web::rpc::{ParamsForCreate, ParamsForUpdate, ParamsIded};
use crate::web::Result;

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
