#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello?name=Jen").await?.print().await?;

    hc.do_get("/hello/meow").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        }),
    );
    req_login.await?.print().await?;

    let req_create_ticket = hc.do_post("/api/tickets", json!({
        "title": "Ticket 1"
    }));
    req_create_ticket.await?.print().await?;
     

    Ok(())
}
