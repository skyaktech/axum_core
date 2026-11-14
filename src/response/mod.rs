use crate::errors::ApiError;
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use serde::Serialize;

/// A successful API response: a status code and data rendered as a JSON body.
///
/// Created with [`success`] (200), [`created`] (201) or [`no_content`] (204, empty body).
#[derive(Debug)]
pub struct ApiSuccess<T> {
    status: StatusCode,
    data: T,
}

impl<T> ApiSuccess<T> {
    pub fn status(&self) -> StatusCode {
        self.status
    }

    /// Returns the wrapped data, e.g. to assert on it when testing a handler.
    pub fn into_inner(self) -> T {
        self.data
    }
}

impl<T: Serialize> IntoResponse for ApiSuccess<T> {
    fn into_response(self) -> Response {
        if self.status == StatusCode::NO_CONTENT {
            return self.status.into_response();
        }

        // Serialize here instead of through `Json`: a status tuple would overwrite
        // the 500 that `Json` produces when serialization fails.
        match serde_json::to_vec(&self.data) {
            Ok(body) => (
                self.status,
                [(header::CONTENT_TYPE, "application/json")],
                body,
            )
                .into_response(),
            Err(err) => ApiError::internal(err).into_response(),
        }
    }
}

/// Response type for API in Axum.
///
/// This type alias is used to standardize the response type for API routes in the application.
/// It represents a `Result` where the success variant contains an [`ApiSuccess`] and the error
/// variant contains an `ApiError`.
///
/// # Examples
///
/// ```
/// use skyak_axum_core::response::{ApiResponse, success};
/// use skyak_axum_core::errors::ApiError;
///
/// async fn example_route() -> ApiResponse<String> {
///     success("Success".to_string())
/// }
///
/// async fn error_route() -> ApiResponse<String> {
///     Err(ApiError::not_found("Resource not found"))
/// }
/// ```
pub type ApiResponse<T> = Result<ApiSuccess<T>, ApiError>;

/// Creates a `200 OK` API response with `data` as the JSON body.
///
/// # Examples
///
/// ```
/// use skyak_axum_core::response::{ApiResponse, success};
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
    Ok(ApiSuccess {
        status: StatusCode::OK,
        data,
    })
}

/// Creates a `201 Created` API response with `data` as the JSON body.
///
/// # Examples
///
/// ```
/// use skyak_axum_core::response::{ApiResponse, created};
///
/// async fn create_user() -> ApiResponse<String> {
///     created("Alice".to_string())
/// }
/// ```
pub fn created<T>(data: T) -> ApiResponse<T> {
    Ok(ApiSuccess {
        status: StatusCode::CREATED,
        data,
    })
}

/// Creates a `204 No Content` API response with an empty body.
///
/// # Examples
///
/// ```
/// use skyak_axum_core::response::{ApiResponse, no_content};
///
/// async fn delete_user() -> ApiResponse<()> {
///     no_content()
/// }
/// ```
pub fn no_content() -> ApiResponse<()> {
    Ok(ApiSuccess {
        status: StatusCode::NO_CONTENT,
        data: (),
    })
}

/// Creates an error API response from an `ApiError`.
///
/// This helper function provides a convenient way to return error responses in route handlers.
/// It wraps the provided `ApiError` in the appropriate `Result` type expected by the API.
///
/// # Examples
///
/// ```
/// use skyak_axum_core::response::{ApiResponse, error, success};
/// use skyak_axum_core::errors::ApiError;
///
/// // Return a not found error
/// async fn handle_not_found() -> ApiResponse<()> {
///     error(ApiError::not_found("User not found"))
/// }
///
/// // Return an unauthorized error
/// async fn handle_unauthorized() -> ApiResponse<String> {
///     error(ApiError::unauthorized("Invalid token"))
/// }
///
/// // Handling a result that might fail
/// fn some_fallible_operation() -> Result<String, std::fmt::Error> {
///   Err(std::fmt::Error)
/// }
///
/// async fn handle_operation() -> ApiResponse<String> {
///     let result = some_fallible_operation();
///     match result {
///         Ok(data) => success(data),
///         Err(err) => error(ApiError::internal(err))
///     }
/// }
/// ```
pub fn error<T>(error: ApiError) -> ApiResponse<T> {
    Err(error)
}
