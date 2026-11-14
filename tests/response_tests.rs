use std::collections::BTreeMap;

use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use http_body_util::BodyExt;
use serde_json::{json, Value};
use skyak_axum_core::{created, error, no_content, success, ApiError, ApiResponse};

#[derive(serde::Serialize)]
struct User {
    id: u32,
    name: String,
}

fn user() -> User {
    User {
        id: 1,
        name: "Alice".to_string(),
    }
}

async fn assert_json_response<T: serde::Serialize>(
    response: ApiResponse<T>,
    status: StatusCode,
    expected: Value,
) {
    let response = response.into_response();
    assert_eq!(response.status(), status);
    assert_eq!(response.headers()[header::CONTENT_TYPE], "application/json");
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body, expected);
}

#[tokio::test]
async fn test_success() {
    assert_json_response(
        success(user()),
        StatusCode::OK,
        json!({ "id": 1, "name": "Alice" }),
    )
    .await;
}

#[tokio::test]
async fn test_created() {
    assert_json_response(
        created(user()),
        StatusCode::CREATED,
        json!({ "id": 1, "name": "Alice" }),
    )
    .await;
}

#[tokio::test]
async fn test_no_content() {
    let response = no_content().into_response();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    assert!(response.headers().get(header::CONTENT_TYPE).is_none());
    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert!(body.is_empty());
}

#[tokio::test]
async fn test_error() {
    let response: ApiResponse<User> = error(ApiError::not_found("Test error"));
    assert_json_response(
        response,
        StatusCode::NOT_FOUND,
        json!({ "code": "not_found", "message": "Test error" }),
    )
    .await;
}

#[tokio::test]
async fn test_serialization_failure_is_internal_error() {
    // JSON object keys must be strings, so this map cannot be serialized.
    let unserializable = || BTreeMap::from([(vec![1u8], 1u32)]);

    for response in [success(unserializable()), created(unserializable())] {
        assert_json_response(
            response,
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "code": "internal_server_error", "message": "Internal Server Error" }),
        )
        .await;
    }
}

#[test]
fn test_accessors() {
    let response = created("Test data".to_string()).unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    assert_eq!(response.into_inner(), "Test data");
}
