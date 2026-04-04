use std::fmt;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

/// Represents common HTTP API errors with optional custom messages.
///
/// This enum provides a standardized way to handle HTTP errors in an Axum web application.
/// Each variant corresponds to a specific HTTP status code and can optionally include
/// a custom error message. When converted to a response, it will use either the provided
/// custom message or a default message appropriate for the error type.
///
/// # Examples
///
/// ```
/// use skyak_axum_core::errors::ApiError;
///
/// // With custom error message
/// let not_found = ApiError::NotFound(Some("User profile not found".to_string()));
///
/// // Without custom message (will use default)
/// let unauthorized = ApiError::Unauthorized(None);
///
/// // Custom status code
/// let teapot = ApiError::Other(418, Some("I'm a teapot".to_string()));
/// ```
#[derive(Serialize, Debug)]
pub enum ApiError {
    BadRequest(Option<String>),
    NotFound(Option<String>),
    InternalServerError(Option<String>),
    Unauthorized(Option<String>),
    Forbidden(Option<String>),
    Conflict(Option<String>),
    UnprocessableEntity(Option<String>),
    TooManyRequests(Option<String>),
    ServiceUnavailable(Option<String>),
    GatewayTimeout(Option<String>),
    Other(u16, Option<String>),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::BadRequest(msg) => write!(f, "{}", msg.as_deref().unwrap_or("Bad Request")),
            ApiError::NotFound(msg) => write!(f, "{}", msg.as_deref().unwrap_or("Not Found")),
            ApiError::InternalServerError(msg) => {
                write!(f, "{}", msg.as_deref().unwrap_or("Internal Server Error"))
            }
            ApiError::Unauthorized(msg) => {
                write!(f, "{}", msg.as_deref().unwrap_or("Unauthorized"))
            }
            ApiError::Forbidden(msg) => write!(f, "{}", msg.as_deref().unwrap_or("Forbidden")),
            ApiError::Conflict(msg) => write!(f, "{}", msg.as_deref().unwrap_or("Conflict")),
            ApiError::UnprocessableEntity(msg) => {
                write!(f, "{}", msg.as_deref().unwrap_or("Unprocessable Entity"))
            }
            ApiError::TooManyRequests(msg) => {
                write!(f, "{}", msg.as_deref().unwrap_or("Too Many Requests"))
            }
            ApiError::ServiceUnavailable(msg) => {
                write!(f, "{}", msg.as_deref().unwrap_or("Service Unavailable"))
            }
            ApiError::GatewayTimeout(msg) => {
                write!(f, "{}", msg.as_deref().unwrap_or("Gateway Timeout"))
            }
            ApiError::Other(_, msg) => write!(f, "{}", msg.as_deref().unwrap_or("Other Error")),
        }
    }
}

impl std::error::Error for ApiError {}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match &self {
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiError::Forbidden(_) => StatusCode::FORBIDDEN,
            ApiError::Conflict(_) => StatusCode::CONFLICT,
            ApiError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            ApiError::TooManyRequests(_) => StatusCode::TOO_MANY_REQUESTS,
            ApiError::ServiceUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
            ApiError::GatewayTimeout(_) => StatusCode::GATEWAY_TIMEOUT,
            ApiError::Other(code, _) => {
                StatusCode::from_u16(*code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
            }
        };

        (status, self.to_string()).into_response()
    }
}
