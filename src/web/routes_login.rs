use crate::{web, Error, Result};
use axum::{routing, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

pub fn routes() -> Router {
    Router::new().route("/api/login", routing::post(api_login))
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

// body/payload extractor has to be last argument

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login ", "HANDLER");

    // TODO: implement real db/auth logic.

    if payload.username != "HIRE" || payload.password != "ME" {
        return Err(Error::LoginFail);
    }

    // FIXME: implement real auth-token with HttpsOnly Cookies
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user1.expiration.signature"));

    //Create the success body

    let body: Json<Value> = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}
