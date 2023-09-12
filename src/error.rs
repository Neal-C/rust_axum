use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFail,

    // --- Auth errors
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailAppCtxNotInRequestExt,

    // --- Model errors
    // Refactor at Model Layer
    TicketDeleteFailedIdNotFound { id: u64 },
}

// Best practice is to implement Display for Error types
// Even when not needed

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {
    // no content
    // no code
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:#?}", "INTO_RESPONSE");

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        response.extensions_mut().insert(self);

        response
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            // Auth
            Self::AuthFailAppCtxNotInRequestExt
            | Self::AuthFailNoAuthTokenCookie
            | Self::AuthFailTokenWrongFormat => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),
            // Model
            Self::TicketDeleteFailedIdNotFound { .. } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }
            //Login
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
// Client error conventions UPPERCASE_SNAKE
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}
