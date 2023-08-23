use std::time::{SystemTime, UNIX_EPOCH};

use crate::{ctx::Ctx, error::ClientError, Error, Result};
use axum::http::{Method, Uri};
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use uuid::Uuid;

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
    uuid: String,
    timestamp: String,

    // ctx
    user_id: Option<u64>,

    // http req attrs
    req_method: String,
    req_path: String,

    // err attrs
    error_type: Option<String>,
    error_data: Option<Value>,
    client_error_type: Option<String>,
}

pub async fn log_request(
    uuid: Uuid,
    req_method: Method,
    uri: Uri,
    ctx: Option<Ctx>,
    service_error: Option<&Error>,
    client_error: Option<ClientError>,
) -> Result<()> {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let error_type = service_error.map(|e| e.as_ref().to_string());
    let error_data = serde_json::to_value(service_error)
        .ok()
        .and_then(|mut v| v.get_mut("data").map(|v| v.take()));

    let log_line = RequestLogLine {
        uuid: uuid.to_string(),
        timestamp: ts.to_string(),

        user_id: ctx.map(|c| c.get_user_id()),

        req_path: uri.to_string(),
        req_method: req_method.to_string(),

        client_error_type: client_error.map(|e| e.as_ref().to_string()),
        error_type,
        error_data,
    };

    // TODO: send this to a log monitor
    println!("->> log_request: \n{}", json!(log_line));

    Ok(())
}
