#![allow(unused)]

use anyhow::Result;
use httpc_test::Response;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    let req_create_bounty = hc.do_post(
        "/api/rpc",
        json!({
            "id": "1",
            "method": "create_bounty",
            "params": {
                "data": {
                    "title" : "AUX-01"
                }
            }
        }),
    );
    req_create_bounty.await?.print().await?;
    let req_update_bounty = hc.do_post(
        "/api/rpc",
        json!({
            "id": "1",
            "method": "update_bounty",
            "params": {
                "id": 1000,
                "data": {
                    "title" : "AUX-02"
                }
            }
        }),
    );
    req_update_bounty.await?.print().await?;

    let req_list_bounties = hc.do_post(
        "/api/rpc",
        json!({
            "id": "1",
            "method": "list_bounty"
        }),
    );
    req_list_bounties.await?.print().await?;
    let req_delete_bounty = hc.do_post(
        "/api/rpc",
        json!({
            "id": "1",
            "method": "delete_bounty",
            "params": {
                "id": 1000,
                "data": {
                    "title" : "AUX-02"
                }
            }
        }),
    );
    req_delete_bounty.await?.print().await?;
    let req_delete_bounty = hc.do_post(
        "/api/rpc",
        json!({
            "id": "1",
            "method": "create_project",
            "params": {
                "data": {
                    "project_id":"AUX",
                    "project_name": "String",
                    "project_lead_address": "String",
                    "project_budget": "String",
                    "project_denom": "String",
                    "project_type": "String",
                    "project_description": "String",
                }
            }
        }),
    );
    req_delete_bounty.await?.print().await?;

    Ok(())
    /*
       let porta = hc.do_get("/porta").await?;

       assert_eq!(porta.status(), 200);

       // assert_eq!(hc.do_get("/api/projects").await?.status(), 500);

       // assert_eq!(hc.do_get("/api/users").await?.status(), 500);

       // Login
       let req_login = hc.do_post(
           "/api/login",
           json!({
               "email": "demo",
               "password": "we",
               "address":"123121"
           }),
       );

       // hc.do_get("/api/projects").await?.print().await?;

       // req_login.await?.print().await?;

       // Create User
       let user1 = hc.do_post(
           "/api/user",
           json!({
               "email": "demo1",
               "password": "welcom",
               "address":"3298420398490238jiweojwoeirjow"
           }),
       );
       user1.await?.print().await?;
       // assert_eq!(user1.await?.status(), 201);
       // Login

       // List Projects
       hc.do_get("/api/projects").await?.print().await?;

       // Create Project
       let project = hc.do_post(
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
       project.await?.print().await?;
       println!("here1");
       let project1_1 = hc.do_patch(
           "/api/projects/AUX4",
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
       let res_pro = hc.do_get("/api/projects/AUX4");
       res_pro.await?.print().await?;

       // let res = hc.do_delete("/api/project/AUX4");

       //    res.await?.print().await?;
       Ok(())
    */
}

// TODO: Write integration tests
