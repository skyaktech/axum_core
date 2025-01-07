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
/// use skyaktech_axum_core::errors::ApiError;
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
#[derive(Serialize)]
pub enum ApiError {
    BadRequest(Option<String>),
    NotFound(Option<String>),
    InternalServerError(Option<String>),
    Unauthorized(Option<String>),
    Forbidden(Option<String>),
    Conflict(Option<String>),
    TooManyRequests(Option<String>),
    ServiceUnavailable(Option<String>),
    GatewayTimeout(Option<String>),
    Other(u16, Option<String>),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            ApiError::BadRequest(_) => (StatusCode::BAD_REQUEST, "Bad Request".to_string()),
            ApiError::NotFound(body) => (StatusCode::NOT_FOUND, body.unwrap_or("Not Found".to_string())),
            ApiError::InternalServerError(body) => (StatusCode::INTERNAL_SERVER_ERROR, body.unwrap_or("Internal Server Error".to_string())),
            ApiError::Unauthorized(body) => (StatusCode::UNAUTHORIZED, body.unwrap_or("Unauthorized".to_string())),
            ApiError::Forbidden(body) => (StatusCode::FORBIDDEN, body.unwrap_or("Forbidden".to_string())),
            ApiError::Conflict(body) => (StatusCode::CONFLICT, body.unwrap_or("Conflict".to_string())),
            ApiError::TooManyRequests(body) => (StatusCode::TOO_MANY_REQUESTS, body.unwrap_or("Too Many Requests".to_string())),
            ApiError::ServiceUnavailable(body) => (StatusCode::SERVICE_UNAVAILABLE, body.unwrap_or("Service Unavailable".to_string())),
            ApiError::GatewayTimeout(body) => (StatusCode::GATEWAY_TIMEOUT, body.unwrap_or("Gateway Timeout".to_string())),
            ApiError::Other(status, body) => (StatusCode::from_u16(status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR), body.unwrap_or("Other Error".to_string())),
        };

        (status, body).into_response()
    }
}
