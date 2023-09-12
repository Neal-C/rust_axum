use anyhow::Result as AnyhowResult;
use serde_json::json;

//cargo install cargo-watch

// cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"

//quiet clear watch test execute 'test quiet quick_dev with argument --nocapture, no capture means we see the println! in terminal

#[tokio::test]
async fn quick_dev() -> AnyhowResult<()> {
    let http_test_client = httpc_test::new_client("http://localhost:8080")?;

    http_test_client
        .do_get("/hello?name=Hire%20me")
        .await?
        .print()
        .await?;

    let request_login = http_test_client.do_post(
        "/api/login",
        json!({
                "username": "HIRE",
                "password": "ME"
        }),
    );

    request_login.await?.print().await?;

    let request_create_ticket = http_test_client.do_post(
        "/api/tickets",
        json!({
        "title": "Hire me"
        }),
    );

    request_create_ticket.await?.print().await?;

    http_test_client
        .do_delete("/api/tickets?id=1")
        .await?
        .print()
        .await?;

    http_test_client
        .do_get("/api/tickets")
        .await?
        .print()
        .await?;

    Ok(())
}
