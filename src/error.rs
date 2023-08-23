use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use tracing::debug;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFail,

    // Auth
    AuthFailNoAuthTokenCookie,
    AuthFailTokenNotValid,
    AuthFailCtxNotFoundInReqExt,

    // Model
    TodoDeleteFailIdNotFound { id: u64 },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        debug!("IntoResponse for Error: {:?}", self);
        
        // create a place holder for Axum response
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // insert the Error into the response
        response.extensions_mut().insert(self);

        response
    }
}

impl Error {
    pub fn map_server_client_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            // login
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

            // auth
            Self::AuthFailTokenNotValid
            | Self::AuthFailCtxNotFoundInReqExt
            | Self::AuthFailNoAuthTokenCookie => (StatusCode::FORBIDDEN, ClientError::NOT_AUTHENTICATED),

            // model
            Self::TodoDeleteFailIdNotFound { .. } => 
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            ,

            // fallback
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::INTERNAL_SERVER_ERROR,
            ),
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NOT_AUTHENTICATED,
    INVALID_PARAMS,
    INTERNAL_SERVER_ERROR,
}
