# Changelog

## Unreleased (0.4.0)

### Breaking

- `ApiError` is an opaque struct instead of an enum; build it with constructors
- Error responses are JSON `{"code", "message"}` instead of plain text
- `ApiError` no longer implements `Serialize`
- `ApiResponse<T>` is `Result<ApiSuccess<T>, ApiError>` instead of `Result<Json<T>, ApiError>`
- Module `https` renamed to `response`
- axum's default features are no longer enabled by this crate and `tokio` is no
  longer a dependency; enable what you use in your own `Cargo.toml`

### Added

- `ApiError` constructors (`not_found`, `bad_request`, ...), `new(StatusCode)`,
  `with_message`, `with_code`, and accessors `status()`, `code()`, `message()`
- `ApiError::internal` and `with_source`: keep the cause for logs and
  `Error::source`, never send it to the client
- `From<JsonRejection>`, `From<QueryRejection>`, `From<PathRejection>` for `ApiError`
- `tracing` feature (default): logs the hidden cause when the response is built
- `ApiSuccess<T>` with `created` (201) and `no_content` (204) helpers
- Main items re-exported at the crate root
- Minimum supported Rust version: 1.80

### Migration

| 0.3 | 0.4 |
|---|---|
| `ApiError::NotFound(Some("x".to_string()))` | `ApiError::not_found("x")` |
| `ApiError::NotFound(None)` | `ApiError::new(StatusCode::NOT_FOUND)` |
| `ApiError::Other(418, Some(m))` | `ApiError::new(StatusCode::IM_A_TEAPOT).with_message(m)` |
| `ApiError::InternalServerError(Some(m))` | `ApiError::internal(err)` — text is no longer sent to the client |
| `match err { ApiError::NotFound(_) => .. }` | `err.status() == StatusCode::NOT_FOUND` |
| `skyak_axum_core::https::…` | `skyak_axum_core::response::…` or the crate root |
| `Ok(Json(x))` | `success(x)` |
| error body `text/plain` | `application/json` `{"code", "message"}` |

## 0.3.0

- `Display` and `std::error::Error` trait implementations for `ApiError`
- `UnprocessableEntity` (422) error variant
- Comprehensive test suite

## 0.2.1

- Fixed `BadRequest` error to include the `message` field in the response body.

## 0.2.0

- `success` and `error` response helpers
- unit tests

## 0.1.0

- Initial release
- Basic Errors and Api response
