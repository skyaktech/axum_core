# skyak_axum_core

Core utilities and extensions for the Axum web framework.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
skyak_axum_core = "0.2.1"
```

## License

This project is licensed under the MIT license.

## Changelog

### 0.2.1
- Fixed `BadRequest` error to include the `message` field in the response body.

### 0.2.0

- `success` and `error` response helpers
- unit tests

### 0.1.0

- Initial release
- Basic Errors and Api response