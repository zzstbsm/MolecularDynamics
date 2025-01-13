use anyhow::Result;
use serde_json::json;

const HOST: &str = "http://localhost:5000";

#[tokio::test]
async fn hello() -> Result<()> {

    let hc = httpc_test::new_client(HOST)?;

    println!("===> Test Olivia");
    hc.do_get("/hello?name=Olivia").await?.print().await?;
    println!("Test Anastasia");
    hc.do_get("/hello2/Anastasia").await?.print().await?;

    // hc.do_get("/backend/src/main.rs").await?.print().await?;

    println!("===> Test successful login");
    hc.do_post("/api/login", json!({
        "username": "demo1",
        "pwd": "welcome"
    })).await?.print().await?;

    println!("===> Test failed login");
    hc.do_post("/api/login", json!({
        "username": "fake",
        "pwd": "nope"
    })).await?.print().await?;

    Ok(())
}

