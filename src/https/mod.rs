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
/// use skyak_axum_core::https::ApiResponse;
/// use skyak_axum_core::errors::ApiError;
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

/// Creates a successful API response by wrapping data in `Json` and `Ok`.
///
/// This helper function simplifies the creation of successful API responses by automatically
/// wrapping the provided data in both `Json` and `Ok`. It's particularly useful in route
/// handlers where you want to return successful responses with less boilerplate.
///
/// # Arguments
///
/// * `data` - The data to be returned in the response. This can be any type that implements
///            serialization traits required by `axum::Json`.
///
/// # Returns
///
/// Returns an `ApiResponse<T>` containing the wrapped data.
///
/// # Examples
///
/// ```
/// use skyak_axum_core::https::{ApiResponse, success};
///
/// // Simple success response with a string
/// async fn handle_string() -> ApiResponse<String> {
///     success("Hello, world!".to_string())
/// }
///
/// // Success response with a complex type
/// #[derive(serde::Serialize)]
/// struct User {
///     id: i32,
///     name: String,
/// }
///
/// async fn handle_user() -> ApiResponse<User> {
///     let user = User {
///         id: 1,
///         name: "Alice".to_string(),
///     };
///     success(user)
/// }
/// ```
pub fn success<T>(data: T) -> ApiResponse<T> {
    Ok(Json(data))
}
