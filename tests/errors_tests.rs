use axum::http::StatusCode;
use axum::response::IntoResponse;
use http_body_util::BodyExt;
use skyak_axum_core::errors::ApiError;

async fn assert_error_response(error: ApiError, expected_status: StatusCode, expected_body: &str) {
    let response = error.into_response();
    assert_eq!(response.status(), expected_status);
    let body = BodyExt::collect(response.into_body())
        .await
        .unwrap()
        .to_bytes();
    assert_eq!(body, expected_body);
}

// --- Default messages (None) ---

#[tokio::test]
async fn test_bad_request_default() {
    assert_error_response(ApiError::BadRequest(None), StatusCode::BAD_REQUEST, "Bad Request").await;
}

#[tokio::test]
async fn test_not_found_default() {
    assert_error_response(ApiError::NotFound(None), StatusCode::NOT_FOUND, "Not Found").await;
}

#[tokio::test]
async fn test_internal_server_error_default() {
    assert_error_response(
        ApiError::InternalServerError(None),
        StatusCode::INTERNAL_SERVER_ERROR,
        "Internal Server Error",
    )
    .await;
}

#[tokio::test]
async fn test_unauthorized_default() {
    assert_error_response(ApiError::Unauthorized(None), StatusCode::UNAUTHORIZED, "Unauthorized")
        .await;
}

#[tokio::test]
async fn test_forbidden_default() {
    assert_error_response(ApiError::Forbidden(None), StatusCode::FORBIDDEN, "Forbidden").await;
}

#[tokio::test]
async fn test_conflict_default() {
    assert_error_response(ApiError::Conflict(None), StatusCode::CONFLICT, "Conflict").await;
}

#[tokio::test]
async fn test_unprocessable_entity_default() {
    assert_error_response(
        ApiError::UnprocessableEntity(None),
        StatusCode::UNPROCESSABLE_ENTITY,
        "Unprocessable Entity",
    )
    .await;
}

#[tokio::test]
async fn test_too_many_requests_default() {
    assert_error_response(
        ApiError::TooManyRequests(None),
        StatusCode::TOO_MANY_REQUESTS,
        "Too Many Requests",
    )
    .await;
}

#[tokio::test]
async fn test_service_unavailable_default() {
    assert_error_response(
        ApiError::ServiceUnavailable(None),
        StatusCode::SERVICE_UNAVAILABLE,
        "Service Unavailable",
    )
    .await;
}

#[tokio::test]
async fn test_gateway_timeout_default() {
    assert_error_response(
        ApiError::GatewayTimeout(None),
        StatusCode::GATEWAY_TIMEOUT,
        "Gateway Timeout",
    )
    .await;
}

#[tokio::test]
async fn test_other_default() {
    assert_error_response(
        ApiError::Other(418, None),
        StatusCode::IM_A_TEAPOT,
        "Other Error",
    )
    .await;
}

// --- Custom messages (Some) ---

#[tokio::test]
async fn test_bad_request_custom() {
    assert_error_response(
        ApiError::BadRequest(Some("Invalid email".to_string())),
        StatusCode::BAD_REQUEST,
        "Invalid email",
    )
    .await;
}

#[tokio::test]
async fn test_not_found_custom() {
    assert_error_response(
        ApiError::NotFound(Some("User not found".to_string())),
        StatusCode::NOT_FOUND,
        "User not found",
    )
    .await;
}

#[tokio::test]
async fn test_internal_server_error_custom() {
    assert_error_response(
        ApiError::InternalServerError(Some("Database connection failed".to_string())),
        StatusCode::INTERNAL_SERVER_ERROR,
        "Database connection failed",
    )
    .await;
}

#[tokio::test]
async fn test_unauthorized_custom() {
    assert_error_response(
        ApiError::Unauthorized(Some("Token expired".to_string())),
        StatusCode::UNAUTHORIZED,
        "Token expired",
    )
    .await;
}

#[tokio::test]
async fn test_forbidden_custom() {
    assert_error_response(
        ApiError::Forbidden(Some("Admin only".to_string())),
        StatusCode::FORBIDDEN,
        "Admin only",
    )
    .await;
}

#[tokio::test]
async fn test_conflict_custom() {
    assert_error_response(
        ApiError::Conflict(Some("Username taken".to_string())),
        StatusCode::CONFLICT,
        "Username taken",
    )
    .await;
}

#[tokio::test]
async fn test_unprocessable_entity_custom() {
    assert_error_response(
        ApiError::UnprocessableEntity(Some("Invalid email format".to_string())),
        StatusCode::UNPROCESSABLE_ENTITY,
        "Invalid email format",
    )
    .await;
}

#[tokio::test]
async fn test_too_many_requests_custom() {
    assert_error_response(
        ApiError::TooManyRequests(Some("Rate limit exceeded".to_string())),
        StatusCode::TOO_MANY_REQUESTS,
        "Rate limit exceeded",
    )
    .await;
}

#[tokio::test]
async fn test_service_unavailable_custom() {
    assert_error_response(
        ApiError::ServiceUnavailable(Some("Maintenance mode".to_string())),
        StatusCode::SERVICE_UNAVAILABLE,
        "Maintenance mode",
    )
    .await;
}

#[tokio::test]
async fn test_gateway_timeout_custom() {
    assert_error_response(
        ApiError::GatewayTimeout(Some("Upstream timed out".to_string())),
        StatusCode::GATEWAY_TIMEOUT,
        "Upstream timed out",
    )
    .await;
}

#[tokio::test]
async fn test_other_custom() {
    assert_error_response(
        ApiError::Other(418, Some("I'm a teapot".to_string())),
        StatusCode::IM_A_TEAPOT,
        "I'm a teapot",
    )
    .await;
}

// --- Edge cases ---

#[tokio::test]
async fn test_other_invalid_status_code() {
    assert_error_response(
        ApiError::Other(9999, Some("bad code".to_string())),
        StatusCode::INTERNAL_SERVER_ERROR,
        "bad code",
    )
    .await;
}

#[tokio::test]
async fn test_other_zero_status_code() {
    assert_error_response(
        ApiError::Other(0, None),
        StatusCode::INTERNAL_SERVER_ERROR,
        "Other Error",
    )
    .await;
}

#[test]
fn test_debug_trait() {
    let error = ApiError::NotFound(Some("gone".to_string()));
    let debug = format!("{:?}", error);
    assert!(debug.contains("NotFound"));
    assert!(debug.contains("gone"));
}

#[test]
fn test_serialize() {
    let error = ApiError::BadRequest(Some("oops".to_string()));
    let json = serde_json::to_string(&error).unwrap();
    assert!(json.contains("BadRequest"));
    assert!(json.contains("oops"));
}

// --- Display + Error traits ---

#[test]
fn test_display_default() {
    assert_eq!(ApiError::BadRequest(None).to_string(), "Bad Request");
    assert_eq!(ApiError::NotFound(None).to_string(), "Not Found");
    assert_eq!(
        ApiError::InternalServerError(None).to_string(),
        "Internal Server Error"
    );
    assert_eq!(ApiError::Unauthorized(None).to_string(), "Unauthorized");
    assert_eq!(ApiError::Forbidden(None).to_string(), "Forbidden");
    assert_eq!(ApiError::Conflict(None).to_string(), "Conflict");
    assert_eq!(
        ApiError::UnprocessableEntity(None).to_string(),
        "Unprocessable Entity"
    );
    assert_eq!(
        ApiError::TooManyRequests(None).to_string(),
        "Too Many Requests"
    );
    assert_eq!(
        ApiError::ServiceUnavailable(None).to_string(),
        "Service Unavailable"
    );
    assert_eq!(ApiError::GatewayTimeout(None).to_string(), "Gateway Timeout");
    assert_eq!(ApiError::Other(418, None).to_string(), "Other Error");
}

#[test]
fn test_display_custom() {
    assert_eq!(
        ApiError::NotFound(Some("user 42".to_string())).to_string(),
        "user 42"
    );
    assert_eq!(
        ApiError::Other(418, Some("teapot".to_string())).to_string(),
        "teapot"
    );
}

#[test]
fn test_error_trait() {
    let error = ApiError::NotFound(None);
    let dyn_error: &dyn std::error::Error = &error;
    assert_eq!(dyn_error.to_string(), "Not Found");
    assert!(dyn_error.source().is_none());
}
