use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello?name=Jon").await?.print().await?;
    hc.do_get("/hello/Jon").await?.print().await?;

    hc.do_get("/robot.txt").await?.print().await?;
    hc.do_get("/src/main.rs").await?.print().await?; // should response with 404

    // Create a user
    let req_login = hc.do_post(
        "/login",
        json!({
          "username": "demo1",
          "password": "welcome"
        }),
    );
    req_login.await?.print().await?;

    // Create a Todo
    hc.do_post(
        "/api/todos",
        json!({
          "content": "Make this stuff!"
        }),
    )
    .await?
    .print()
    .await?;

    // List Todos
    hc.do_get("/api/todos").await?.print().await?;

    Ok(())
}
