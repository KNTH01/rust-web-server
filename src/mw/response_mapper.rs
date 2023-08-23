use crate::{ctx::Ctx, Error};
use axum::http::{Method, Uri};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use tracing::debug;
use uuid::Uuid;

use crate::logs::log_request;

pub async fn response_mapper_main(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    debug!("response_mapper_main");

    let uuid = Uuid::new_v4();

    let server_error = res.extensions().get::<Error>();

    let client_status_error =
        server_error.map(|status_error| status_error.map_server_client_error());

    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let error_json = json!({
            "error": {
                "type": client_error.as_ref(),
                "req_id": uuid.to_string(),
                }
            });

            // build the response from the error_json
            (*status_code, Json(error_json)).into_response()
        });

    let client_error = client_status_error.unzip().1;

    log_request(uuid, req_method, uri, ctx, server_error, client_error)
        .await
        .ok();

    error_response.unwrap_or(res)
}
