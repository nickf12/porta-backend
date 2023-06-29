#![allow(unused)]

use anyhow::Result;
//use crate::{model::{Project}};
use serde_json::json;


#[tokio::test]
async fn post_project() -> Result<()> {
    let http_client = httpc_test::new_client("http://localhost:8000/api")?;
    http_client
        .do_post(
            "/projects",
            json!({"title":"una prova", "content": "il test"}),
        )
        .await?
        .print()
        .await;

    http_client.do_get("/projects").await?.print().await;
    Ok(())
}
