use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::FromRow;

use super::base::{self, DbBmc};

// region:    --- Bounty Types
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Bounty {
    pub id: i64,
    pub bounty_title: String,
    pub bounty_assignee: String,
    pub bounty_reward: String,
    pub bounty_deliverables: String,
    pub bounty_description: String,
}
#[derive(Deserialize, Fields)]
pub struct BountyForCreate {
    pub bounty_title: String,
    pub bounty_assignee: String,
    pub bounty_reward: String,
    pub bounty_deliverables: String,
    pub bounty_description: String,
}
#[derive(Deserialize, Fields)]
pub struct BountyForUpdate {
    pub bounty_title: Option<String>,
    pub bounty_assignee: Option<String>,
    pub bounty_reward: Option<String>,
    pub bounty_deliverables: Option<String>,
    pub bounty_description: Option<String>,
}
// endregion: --- Bounty Types

// region:    --- BountyBmc
pub struct BountyBmc;

impl DbBmc for BountyBmc {
    const TABLE: &'static str = "bounty";
}

impl BountyBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, bounty_c: BountyForCreate) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, bounty_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Bounty> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Bounty>> {
        base::list::<Self, _>(ctx, mm).await
    }
    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        bounty_u: BountyForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, bounty_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}
// endregion: --- BountyBmc

// region:       --- UnitTest
#[cfg(test)]
mod tests {
    #![allow(unused)]
    use crate::_dev_utils;

    use super::*;
    use anyhow::Result;
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        // -- Setup & Fixture
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_title = "test_create_ok title";
        // -- Exec
        let bounty_c = BountyForCreate {
            bounty_title: fx_title.to_string(),
            bounty_assignee: fx_title.to_string(),
            bounty_deliverables: fx_title.to_string(),
            bounty_description: fx_title.to_string(),
            bounty_reward: fx_title.to_string(),
        };
        let id = BountyBmc::create(&ctx, &mm, bounty_c).await?;

        // -- Check title using SQL query
        let (title,): (String,) = sqlx::query_as("SELECT title from bounty where id = $1")
            .bind(id)
            .fetch_one(mm.db())
            .await?;

        assert_eq!(title, fx_title);
        // -- Clean Bounty using SQL query
        let count = sqlx::query("DELETE FROM bounty WHERE id = $1")
            .bind(id)
            .execute(mm.db())
            .await?
            .rows_affected();
        assert_eq!(count, 1, "Did not delete 1 row?");
        let bounty_c = BountyForCreate {
            bounty_title: fx_title.to_string(),
            bounty_assignee: fx_title.to_string(),
            bounty_deliverables: fx_title.to_string(),
            bounty_description: fx_title.to_string(),
            bounty_reward: fx_title.to_string(),
        };
        let id = BountyBmc::create(&ctx, &mm, bounty_c).await?;
        // -- Check Bounty using model get method
        let bounty = BountyBmc::get(&ctx, &mm, id).await?;
        assert_eq!(bounty.bounty_title, fx_title);
        // -- Clean Bounty using model delete method
        BountyBmc::delete(&ctx, &mm, id).await?;

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_get_err_not_found() -> Result<()> {
        // -- Setup & Fixture
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let res = BountyBmc::get(&ctx, &mm, 99).await;
        let error = Error::EntityNotFound {
            entity: "bounty",
            id: 99,
        };

        assert!(matches!(
            res,
            Err(Error::EntityNotFound {
                entity: "bounty",
                id: 99,
            })
        ),);
        Ok(())
    }
    #[serial]
    #[tokio::test]
    async fn test_delete_bounty_ok() -> Result<()> {
        // -- Setup & Fixture
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 99;
        // -- Exec
        let res = BountyBmc::delete(&ctx, &mm, fx_id).await;
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "bounty",
                    id: 99,
                })
            ),
            "EntityNotFound not matching"
        );
        Ok(())
    }
    #[serial]
    #[tokio::test]
    async fn test_list_ok() -> Result<()> {
        // -- Setup & Fixture

        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_titles = &["test_list_ok-bounty 01", "test_list_ok-bounty 02"];
        _dev_utils::seed_tasks(&ctx, &mm, fx_titles).await?;
        // -- Exec
        let bounties = BountyBmc::list(&ctx, &mm).await?;
        // -- Check
        let bounties: Vec<Bounty> = bounties
            .into_iter()
            .filter(|t| t.bounty_title.starts_with("test_list_ok"))
            .collect();
        assert_eq!(bounties.len(), 2, "number of seeded bounties");
        // -- Clean
        for bounty in bounties.iter() {
            BountyBmc::delete(&ctx, &mm, bounty.id).await?;
        }
        Ok(())
    }
    #[serial]
    #[tokio::test]
    async fn test_update_ok() -> Result<()> {
        // -- Setup & Fixture
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_title = "test_update_ok - task 01";
        let fx_new_title = "test_update_ok - task 02 - new";
        let fx_bounty = _dev_utils::seed_tasks(&ctx, &mm, &[fx_title])
            .await?
            .remove(0);
        // -- Exec
        BountyBmc::update(
            &ctx,
            &mm,
            fx_bounty.id,
            BountyForUpdate {
                bounty_title: Some(fx_new_title.to_string()),
                bounty_assignee: Some(fx_new_title.to_string()),
                bounty_deliverables: Some(fx_new_title.to_string()),
                bounty_description: Some(fx_new_title.to_string()),
                bounty_reward: Some(fx_new_title.to_string()),
            },
        )
        .await?;
        // -- Check
        let bounty = BountyBmc::get(&ctx, &mm, fx_bounty.id).await?;
        assert_eq!(bounty.bounty_title, fx_new_title);

        Ok(())
    }
}
// Endregion:    --- UnitTest
