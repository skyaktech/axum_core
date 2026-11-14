use std::error::Error;

use axum::body::Body;
use axum::extract::rejection::PathRejection;
use axum::extract::{FromRequest, Path, Query};
use axum::http::{header, Request, StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use skyak_axum_core::errors::ApiError;
use tower::ServiceExt;

async fn assert_response(response: Response, status: StatusCode, code: &str, message: &str) {
    assert_eq!(response.status(), status);
    assert_eq!(response.headers()[header::CONTENT_TYPE], "application/json");
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body, json!({ "code": code, "message": message }));
}

async fn assert_error_response(error: ApiError, status: StatusCode, code: &str, message: &str) {
    assert_response(error.into_response(), status, code, message).await;
}

// --- Constructors ---

#[tokio::test]
async fn test_constructors() {
    let cases = [
        (
            ApiError::bad_request("Invalid email"),
            StatusCode::BAD_REQUEST,
            "bad_request",
            "Invalid email",
        ),
        (
            ApiError::unauthorized("Token expired"),
            StatusCode::UNAUTHORIZED,
            "unauthorized",
            "Token expired",
        ),
        (
            ApiError::forbidden("Admin only"),
            StatusCode::FORBIDDEN,
            "forbidden",
            "Admin only",
        ),
        (
            ApiError::not_found("User not found"),
            StatusCode::NOT_FOUND,
            "not_found",
            "User not found",
        ),
        (
            ApiError::conflict("Username taken"),
            StatusCode::CONFLICT,
            "conflict",
            "Username taken",
        ),
        (
            ApiError::unprocessable_entity("Invalid email format"),
            StatusCode::UNPROCESSABLE_ENTITY,
            "unprocessable_entity",
            "Invalid email format",
        ),
        (
            ApiError::too_many_requests("Rate limit exceeded"),
            StatusCode::TOO_MANY_REQUESTS,
            "too_many_requests",
            "Rate limit exceeded",
        ),
        (
            ApiError::service_unavailable("Maintenance mode"),
            StatusCode::SERVICE_UNAVAILABLE,
            "service_unavailable",
            "Maintenance mode",
        ),
        (
            ApiError::gateway_timeout("Upstream timed out"),
            StatusCode::GATEWAY_TIMEOUT,
            "gateway_timeout",
            "Upstream timed out",
        ),
    ];

    for (error, status, code, message) in cases {
        assert_error_response(error, status, code, message).await;
    }
}

#[tokio::test]
async fn test_new_derives_code_and_message_from_status() {
    assert_error_response(
        ApiError::new(StatusCode::NOT_FOUND),
        StatusCode::NOT_FOUND,
        "not_found",
        "Not Found",
    )
    .await;
    assert_error_response(
        ApiError::new(StatusCode::IM_A_TEAPOT),
        StatusCode::IM_A_TEAPOT,
        "im_a_teapot",
        "I'm a teapot",
    )
    .await;
    assert_error_response(
        ApiError::new(StatusCode::NON_AUTHORITATIVE_INFORMATION),
        StatusCode::NON_AUTHORITATIVE_INFORMATION,
        "non_authoritative_information",
        "Non Authoritative Information",
    )
    .await;
}

#[tokio::test]
async fn test_new_non_standard_status() {
    let status = StatusCode::from_u16(599).unwrap();
    assert_error_response(ApiError::new(status), status, "error", "Error").await;
}

#[tokio::test]
async fn test_with_code_and_message() {
    assert_error_response(
        ApiError::conflict("Email already registered").with_code("email_taken"),
        StatusCode::CONFLICT,
        "email_taken",
        "Email already registered",
    )
    .await;
    assert_error_response(
        ApiError::new(StatusCode::IM_A_TEAPOT).with_message("Short and stout"),
        StatusCode::IM_A_TEAPOT,
        "im_a_teapot",
        "Short and stout",
    )
    .await;
}

// --- Hidden source ---

#[tokio::test]
async fn test_internal_hides_source_from_client() {
    let error = ApiError::internal(std::io::Error::other("db password rejected"));
    assert_eq!(error.source().unwrap().to_string(), "db password rejected");

    assert_error_response(
        error,
        StatusCode::INTERNAL_SERVER_ERROR,
        "internal_server_error",
        "Internal Server Error",
    )
    .await;
}

#[tokio::test]
async fn test_with_source_hides_source_from_client() {
    let error =
        ApiError::gateway_timeout("Upstream timed out").with_source("10.0.0.7:5432 timeout");
    assert_eq!(error.source().unwrap().to_string(), "10.0.0.7:5432 timeout");

    assert_error_response(
        error,
        StatusCode::GATEWAY_TIMEOUT,
        "gateway_timeout",
        "Upstream timed out",
    )
    .await;
}

#[test]
fn test_internal_with_question_mark() {
    fn parse(input: &str) -> Result<u32, ApiError> {
        let number = input.parse::<u32>().map_err(ApiError::internal)?;
        Ok(number)
    }

    assert_eq!(parse("42").unwrap(), 42);
    let error = parse("nope").unwrap_err();
    assert_eq!(error.status(), StatusCode::INTERNAL_SERVER_ERROR);
    assert!(error.source().is_some());
}

// --- Accessors + traits ---

#[test]
fn test_accessors() {
    let error = ApiError::not_found("user 42").with_code("user_missing");
    assert_eq!(error.status(), StatusCode::NOT_FOUND);
    assert_eq!(error.code(), "user_missing");
    assert_eq!(error.message(), "user 42");
}

#[test]
fn test_display_is_message() {
    assert_eq!(ApiError::not_found("user 42").to_string(), "user 42");
    assert_eq!(
        ApiError::new(StatusCode::NOT_FOUND).to_string(),
        "Not Found"
    );
}

#[test]
fn test_error_trait_without_source() {
    let error = ApiError::not_found("gone");
    let dyn_error: &dyn Error = &error;
    assert_eq!(dyn_error.to_string(), "gone");
    assert!(dyn_error.source().is_none());
}

#[test]
fn test_debug_trait() {
    let debug = format!("{:?}", ApiError::not_found("gone"));
    assert!(debug.contains("404"));
    assert!(debug.contains("gone"));
}

// --- Rejections ---

fn json_request(content_type: Option<&str>, body: &'static str) -> Request<Body> {
    let mut request = Request::post("/");
    if let Some(content_type) = content_type {
        request = request.header(header::CONTENT_TYPE, content_type);
    }
    request.body(Body::from(body)).unwrap()
}

async fn json_rejection(request: Request<Body>) -> ApiError {
    Json::<Vec<u32>>::from_request(request, &())
        .await
        .unwrap_err()
        .into()
}

#[tokio::test]
async fn test_json_rejection_syntax_error() {
    let error = json_rejection(json_request(Some("application/json"), "[1,")).await;
    assert_eq!(error.status(), StatusCode::BAD_REQUEST);
    assert_eq!(error.code(), "bad_request");
    assert!(error.message().contains("Failed to parse"));
}

#[tokio::test]
async fn test_json_rejection_data_error() {
    let error = json_rejection(json_request(Some("application/json"), r#"{"a":1}"#)).await;
    assert_eq!(error.status(), StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(error.code(), "unprocessable_entity");
    assert!(error.message().contains("Failed to deserialize"));
}

#[tokio::test]
async fn test_json_rejection_missing_content_type() {
    let error = json_rejection(json_request(None, "[1]")).await;
    assert_eq!(error.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
    assert_eq!(error.code(), "unsupported_media_type");
}

#[tokio::test]
async fn test_query_rejection() {
    #[derive(Debug, serde::Deserialize)]
    struct Params {
        #[allow(dead_code)]
        page: u32,
    }

    let uri: Uri = "/items?page=abc".parse().unwrap();
    let error: ApiError = Query::<Params>::try_from_uri(&uri).unwrap_err().into();
    assert_eq!(error.status(), StatusCode::BAD_REQUEST);
    assert_eq!(error.code(), "bad_request");
    assert!(error
        .message()
        .contains("Failed to deserialize query string"));
}

async fn get_path(app: Router, uri: &str) -> Response {
    app.oneshot(Request::get(uri).body(Body::empty()).unwrap())
        .await
        .unwrap()
}

#[tokio::test]
async fn test_path_rejection_client_error() {
    async fn handler(path: Result<Path<u32>, PathRejection>) -> Result<String, ApiError> {
        let Path(id) = path?;
        Ok(id.to_string())
    }
    let app = Router::new().route("/users/{id}", get(handler));

    let response = get_path(app.clone(), "/users/7").await;
    assert_eq!(response.status(), StatusCode::OK);

    let response = get_path(app, "/users/abc").await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["code"], "bad_request");
    assert!(body["message"].as_str().unwrap().contains("abc"));
}

#[tokio::test]
async fn test_path_rejection_server_error_is_hidden() {
    // Handler expects two params, route only has one: a programming error (500).
    async fn handler(path: Result<Path<(u32, u32)>, PathRejection>) -> Result<String, ApiError> {
        let Path((a, b)) = path?;
        Ok(format!("{a}{b}"))
    }
    let app = Router::new().route("/users/{id}", get(handler));

    assert_response(
        get_path(app, "/users/7").await,
        StatusCode::INTERNAL_SERVER_ERROR,
        "internal_server_error",
        "Internal Server Error",
    )
    .await;
}
