#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    hc.do_get("/porta").await?.print().await?;
    hc.do_get("/api/projects").await?.print().await?;

    hc.do_get("/api/users").await?.print().await?;

    // Login
    let req_login = hc.do_post(
        "/api/login",
        json!({
            "email": "demo1",
            "password": "welcom",
            "address":"3298420398490238jiweojwoeirjow"
        }),
    );
    hc.do_get("/api/projects").await?.print().await?;

    req_login.await?.print().await?;

    // Create User
    let user1 = hc.do_post(
        "/api/users",
        json!({
            "email": "demo1",
            "password": "welcom",
            "address":"3298420398490238jiweojwoeirjow"
        }),
    );
    user1.await?.print().await?;
    // Login
    let req_login = hc.do_post(
        "/api/login",
        json!({
            "email": "demo1",
            "password": "welcom",
            "address":"3298420398490238jiweojwoeirjow"
        }),
    );
    req_login.await?.print().await?;
    // List Projects
    hc.do_get("/api/projects").await?.print().await?;

    // Create Project
    let project1 = hc.do_post(
        "/api/projects",
        json!({
             "project_id": "AUX4",
             "project_name": "Pecunia Inception",
             "project_lead_address": "12345iawjejioj",
             "project_lead_reward": 10,
             "project_budget": 10.2,
             "project_denom": "USDC",
             "project_type": "AUX",
             "project_description": "Inception pecunia finance",
             "project_deliverables": "",
        }),
    );
    project1.await?.print().await?;

    // Edit Project
    let project1_1 = hc.do_patch(
        "/api/project/?id=AUX4",
        json!({
             "project_name": "Pecunia Inception2",
             "project_lead_address": "arewfaewfawfwae",
             "project_lead_reward": 10,
             "project_budget": 10.2,
             "project_denom": "XMR",
             "project_type": "AUX",
             "project_description": "Inception pecunia finance",
             "project_deliverables": "",
        }),
    );
    project1_1.await?.print().await?;

    Ok(())
}
