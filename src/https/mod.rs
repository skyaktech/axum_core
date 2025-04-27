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
///   serialization traits required by `axum::Json`.
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

/// Creates an error API response from an `ApiError`.
///
/// This helper function provides a convenient way to return error responses in route handlers.
/// It wraps the provided `ApiError` in the appropriate `Result` type expected by the API.
///
/// # Arguments
///
/// * `error` - The `ApiError` instance representing the error condition.
///
/// # Returns
///
/// Returns an `ApiResponse<T>` containing the error.
///
/// # Examples
///
/// ```
/// use skyak_axum_core::https::{ApiResponse, error, success};
/// use skyak_axum_core::errors::ApiError;
///
/// // Return a not found error
/// async fn handle_not_found() -> ApiResponse<()> {
///     error(ApiError::NotFound(Some("User not found".to_string())))
/// }
///
/// // Return an unauthorized error
/// async fn handle_unauthorized() -> ApiResponse<String> {
///     error(ApiError::Unauthorized(Some("Invalid token".to_string())))
/// }
///
/// // Handling a result that might fail
/// fn some_fallible_operation() -> Result<String, ()> {
///   Err(())
/// }
///
/// async fn handle_operation() -> ApiResponse<String> {
///     let result = some_fallible_operation();
///     match result {
///         Ok(data) => success(data),
///         Err(_) => error(ApiError::InternalServerError(None))
///     }
/// }
/// ```
pub fn error<T>(error: ApiError) -> ApiResponse<T> {
    Err(error)
}
