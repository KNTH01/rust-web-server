use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookies, Cookie};

use crate::{web, Error, Result};

pub fn routes() -> Router {
    Router::new().route("/login", post(login))
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

async fn login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:12} - login", "HANDLER");

    // TODO: implement real db/auth logic
    if payload.username != "demo1" || payload.password != "welcome" {
        return Err(Error::LoginFail);
    }

    // TODO: Implement real auth-token generation/signature
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));
    
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}
