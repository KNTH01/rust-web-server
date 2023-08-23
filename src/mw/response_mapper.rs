use crate::{ctx::Ctx, Error};
use axum::http::{Method, Uri};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use uuid::Uuid;

use crate::logs::log_request;

pub async fn response_mapper_main(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    let uuid = Uuid::new_v4();
    
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|status_error| status_error.client_status_error());
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let error_json = json!({
            "error": {
                "type": client_error.as_ref(),
                "req_id": uuid.to_string(),
                }
            });

            println!("->> {:<12} - client error body: {error_json}", "RES_MAPPER");

            // build the response from the error_json
            (*status_code, Json(error_json)).into_response()
        });
    let client_error = client_status_error.unzip().1;

    log_request(uuid, req_method, uri, ctx, service_error, client_error)
        .await
        .ok();

    println!();

    error_response.unwrap_or(res)
}
