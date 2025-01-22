use anyhow::Result;
use serde_json::json;

const HOST: &str = "http://localhost:5000";

#[tokio::test]
async fn hello() -> Result<()> {

    let hc = httpc_test::new_client(HOST)?;

    println!("===> Test noname");
    hc.do_get("/hello").await?.print().await?;
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

    println!("===> Test valid payload");
    hc.do_post("/simulation/management/set", json!({
        "number_of_atoms": 200,
        "box_length": 20,
        "current_time": 0,
        "integration_step": 1e-3,
        "target_temperature": 1,
        "lattice_type": "FCC",
        "integrator": "Verlet",
    })).await?.print().await?;

    println!("===> Test invalid lattice");
    hc.do_post("/simulation/management/set", json!({
        "number_of_atoms": 200,
        "box_length": 20,
        "current_time": 0,
        "integration_step": 1e-3,
        "target_temperature": 1,
        "lattice_type": "garbage",
        "integrator": "Verlet",
    })).await?.print().await?;

    println!("===> Test invalid integrator");
    hc.do_post("/simulation/management/set", json!({
        "number_of_atoms": 200,
        "box_length": 20,
        "current_time": 0,
        "integration_step": 1e-3,
        "target_temperature": 1,
        "lattice_type": "FCC",
        "integrator": "garbage",
    })).await?.print().await?;

    println!("===> Test invalid payload");
    hc.do_post("/simulation/management/set", json!({
        "user": "cos"
    })).await?.print().await?;

    Ok(())
}

