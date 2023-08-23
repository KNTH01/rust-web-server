use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

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
        println!("->> {:<12} - {self:?}", "INTO_RES");

        // create a place holder for Axum response
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // insert the Error into the response
        response.extensions_mut().insert(self);

        response
    }
}

impl Error {
    pub fn client_status_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            // login
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

            // auth
            Self::AuthFailTokenNotValid
            | Self::AuthFailCtxNotFoundInReqExt
            | Self::AuthFailNoAuthTokenCookie => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

            // model
            Self::TodoDeleteFailIdNotFound { .. } => 
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            ,

            // fallback
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}
