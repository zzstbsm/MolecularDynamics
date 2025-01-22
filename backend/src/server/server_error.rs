use axum_macros::FromRequest;
use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{
        IntoResponse,
        Response
    }
};
use serde::Serialize;

/// List of all the errors handled by the server
#[derive(Debug)]
pub enum ServerError {
    LoginFail,
    UnhandledError,
    UnprocessableEntity,
}

impl ServerError {
    
    /// Get the status code of all the errors handled by the server
    /// This method is supposed to handle all the errors (no default case allowed)
    fn get_status_code(&self) -> StatusCode {
        match self {
            Self::LoginFail => {
                StatusCode::UNAUTHORIZED
            }
            Self::UnhandledError => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::UnprocessableEntity => {
                StatusCode::BAD_REQUEST
            }
        }
    }

    /// Get the status code associated to a server error
    fn get_server_error(status_code: StatusCode) -> ServerError {
        match status_code {
            StatusCode::UNPROCESSABLE_ENTITY => {
                ServerError::UnprocessableEntity
            }
            _ => {
                ServerError::UnhandledError
            }
        }
    }
}

/// Handle the messages of all errors
impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        println!("->> {:12} - {self:?}", "INTO_RES");

        let message = match self {
            ServerError::LoginFail => {
                "Unauthorized access"
            }
            ServerError::UnprocessableEntity => {
                "Unprocessable entity, please verify the body of the API call"
            }
            _ => {
                "UNHANDLED_CLIENT_ERROR"
            }
        };

        (self.get_status_code(),message).into_response()
    
    }
}

// create an extractor that internally uses `axum::Json` but has a custom rejection
#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(ApiError))]
pub struct CustomJson<T>(pub T);

// We implement `IntoResponse` for our extractor so it can be used as a response
impl<T: Serialize> IntoResponse for CustomJson<T> {
    fn into_response(self) -> axum::response::Response {
        let Self(value) = self;
        axum::Json(value).into_response()
    }
}

/// Intercept the errors handled by the server by default and handle it manually
#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
}

/// Handle the error caused by a bad API Json body
impl From<JsonRejection> for ApiError {
    fn from(rejection: JsonRejection) -> Self {
        Self {
            status: rejection.status(),
        }
    }
}

/// Implement into response for ApiError handling it using manually handled server errors
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        ServerError::get_server_error(self.status)
            .into_response()
    }
}
