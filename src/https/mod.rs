use crate::errors::ApiError;
use axum::Json;

/// Response type for API in Axum.
///
/// This type alias is used to standardize the response type for API routes in the application.
/// It represents a `Result` where the success variant contains a JSON response and the error
/// variant contains an `ApiError`.
///
/// # Examples
///
/// ```
/// use skyaktech_axum_core::https::ApiResponse;
/// use skyaktech_axum_core::errors::ApiError;
/// use axum::Json;
///
/// async fn example_route() -> ApiResponse<String> {
///     Ok(Json("Success".to_string()))
/// }
///
/// async fn error_route() -> ApiResponse<String> {
///     Err(ApiError::NotFound(Some("Resource not found".to_string())))
/// }
/// ```
pub type ApiResponse<T> = Result<Json<T>, ApiError>;
