# skyak_axum_core

Core utilities and extensions for the Axum web framework: one return type for
handlers and JSON errors with machine-readable codes.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
skyak_axum_core = "0.3.0"
```

## Usage

### Handlers

Return `ApiResponse<T>` from every handler and pick the status with a helper.

```rust
use axum::extract::Path;
use axum::routing::{get, post};
use axum::Router;
use serde::Serialize;
use skyak_axum_core::{created, no_content, success, ApiError, ApiResponse};

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
}

// 200 with JSON body
async fn get_user(Path(id): Path<u32>) -> ApiResponse<User> {
    if id != 1 {
        return Err(ApiError::not_found("User not found"));
    }
    success(User { id, name: "Alice".to_string() })
}

// 201 with JSON body
async fn create_user() -> ApiResponse<User> {
    created(User { id: 2, name: "Bob".to_string() })
}

// 204 with empty body
async fn delete_user() -> ApiResponse<()> {
    no_content()
}

let app: Router = Router::new()
    .route("/users", post(create_user))
    .route("/users/{id}", get(get_user).delete(delete_user));
```

### Errors

Every `ApiError` is rendered as JSON with a machine-readable `code` and a
human-readable `message`:

```json
{ "code": "not_found", "message": "User not found" }
```

```rust
use axum::http::StatusCode;
use skyak_axum_core::ApiError;

// 404, code "not_found"
ApiError::not_found("User not found");

// 409, custom code "email_taken"
ApiError::conflict("Email already registered").with_code("email_taken");

// Any status; code and message derived from it ("im_a_teapot" / "I'm a teapot")
ApiError::new(StatusCode::IM_A_TEAPOT);
```

Constructors: `bad_request`, `unauthorized`, `forbidden`, `not_found`, `conflict`,
`unprocessable_entity`, `too_many_requests`, `service_unavailable`, `gateway_timeout`.

### Internal errors

`ApiError::internal` turns any error into a 500. The cause is never sent to the
client, which only receives
`{ "code": "internal_server_error", "message": "Internal Server Error" }`.

```rust
use skyak_axum_core::{success, ApiError, ApiResponse};

async fn read_config() -> ApiResponse<String> {
    let config = std::fs::read_to_string("config.toml").map_err(ApiError::internal)?;
    success(config)
}
```

The cause stays available through `std::error::Error::source` and is logged with
`tracing::error!` when the response is built. Use `with_source` to attach a hidden
cause to any other error, e.g.
`ApiError::gateway_timeout("Upstream timed out").with_source(err)`.

### Extractor rejections

`ApiError` implements `From` for axum's `JsonRejection`, `QueryRejection` and
`PathRejection`, so invalid input gets the same JSON error format:

```rust
use axum::extract::rejection::JsonRejection;
use axum::Json;
use serde::Deserialize;
use skyak_axum_core::{created, ApiResponse};

#[derive(Deserialize)]
struct NewUser {
    name: String,
}

async fn create_user(payload: Result<Json<NewUser>, JsonRejection>) -> ApiResponse<String> {
    let Json(new_user) = payload?;
    created(new_user.name)
}
```

## Features

- `tracing` (default) — log the hidden cause of an error when it is turned into a
  response. Disable with `default-features = false`.

## License

This project is licensed under the MIT license.

## Changelog

See [CHANGELOG.md](CHANGELOG.md).
