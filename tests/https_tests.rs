use skyak_axum_core::https;
use skyak_axum_core::errors;

#[tokio::test]
async fn test_success() {
    let data = "Test data".to_string();
    let response = https::success(data.clone());
    assert!(response.is_ok());
    assert_eq!(response.unwrap().0, data);
}

#[tokio::test]
async fn test_error() {
    let error_message = "Test error";
    let error = errors::ApiError::NotFound(Some(error_message.to_string()));
    let response: https::ApiResponse<String> = https::error(error);

    assert!(response.is_err());

    match response.unwrap_err() {
        errors::ApiError::NotFound(Some(msg)) => assert_eq!(msg, error_message),
        _ => panic!("Expected NotFound error variant"),
    }
}