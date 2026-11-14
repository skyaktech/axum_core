use skyak_axum_core::errors;
use skyak_axum_core::https;

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
    let error = errors::ApiError::not_found(error_message);
    let response: https::ApiResponse<String> = https::error(error);

    assert!(response.is_err());
    assert_eq!(response.unwrap_err().message(), error_message);
}
