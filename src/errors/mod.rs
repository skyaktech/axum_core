use std::error::Error;
use std::fmt;

use axum::extract::rejection::{JsonRejection, PathRejection, QueryRejection};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

type BoxError = Box<dyn Error + Send + Sync>;

/// An HTTP API error rendered as a JSON body with a machine-readable `code`
/// and a human-readable `message`.
///
/// The status code decides the default `code` and `message`; both can be overridden.
/// An optional source error is kept for logging and [`Error::source`], and is never
/// sent to the client.
///
/// # Examples
///
/// ```
/// use axum::http::StatusCode;
/// use skyak_axum_core::errors::ApiError;
///
/// // {"code":"not_found","message":"User profile not found"}
/// let not_found = ApiError::not_found("User profile not found");
///
/// // Default message: {"code":"unauthorized","message":"Unauthorized"}
/// let unauthorized = ApiError::new(StatusCode::UNAUTHORIZED);
///
/// // Custom code: {"code":"email_taken","message":"Email already registered"}
/// let conflict = ApiError::conflict("Email already registered").with_code("email_taken");
///
/// // The cause is logged, the client only sees a generic 500
/// let internal = ApiError::internal(std::io::Error::other("connection reset"));
/// ```
#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    code: String,
    message: String,
    source: Option<BoxError>,
}

macro_rules! constructors {
    ($($name:ident => $status:ident),* $(,)?) => {$(
        #[doc = concat!("Creates a `", stringify!($status), "` error with the given message.")]
        pub fn $name(message: impl Into<String>) -> Self {
            Self::new(StatusCode::$status).with_message(message)
        }
    )*};
}

impl ApiError {
    /// Creates an error whose code and message are derived from the status code,
    /// e.g. `404` gives `not_found` / `Not Found`.
    pub fn new(status: StatusCode) -> Self {
        let message = status.canonical_reason().unwrap_or("Error");
        let code = message
            .chars()
            .filter_map(|c| match c {
                ' ' | '-' => Some('_'),
                c if c.is_ascii_alphanumeric() => Some(c.to_ascii_lowercase()),
                _ => None,
            })
            .collect();

        Self {
            status,
            code,
            message: message.to_string(),
            source: None,
        }
    }

    constructors! {
        bad_request => BAD_REQUEST,
        unauthorized => UNAUTHORIZED,
        forbidden => FORBIDDEN,
        not_found => NOT_FOUND,
        conflict => CONFLICT,
        unprocessable_entity => UNPROCESSABLE_ENTITY,
        too_many_requests => TOO_MANY_REQUESTS,
        service_unavailable => SERVICE_UNAVAILABLE,
        gateway_timeout => GATEWAY_TIMEOUT,
    }

    /// Creates an `INTERNAL_SERVER_ERROR` that keeps `source` for logging
    /// while the client only receives the generic message.
    ///
    /// ```
    /// use skyak_axum_core::errors::ApiError;
    ///
    /// fn parse(input: &str) -> Result<u32, ApiError> {
    ///     input.parse().map_err(ApiError::internal)
    /// }
    /// ```
    pub fn internal(source: impl Into<BoxError>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR).with_source(source)
    }

    /// Replaces the message sent to the client.
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    /// Replaces the machine-readable code sent to the client.
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = code.into();
        self
    }

    /// Attaches the underlying cause. It is logged, never sent to the client.
    pub fn with_source(mut self, source: impl Into<BoxError>) -> Self {
        self.source = Some(source.into());
        self
    }

    pub fn status(&self) -> StatusCode {
        self.status
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for ApiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_deref()
            .map(|source| source as &(dyn Error + 'static))
    }
}

#[derive(Serialize)]
struct Body {
    code: String,
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        #[cfg(feature = "tracing")]
        if let Some(source) = &self.source {
            tracing::error!(status = self.status.as_u16(), code = %self.code, error = %source, "api error");
        }

        let body = Body {
            code: self.code,
            message: self.message,
        };

        (self.status, Json(body)).into_response()
    }
}

macro_rules! from_rejection {
    ($($rejection:ty),*) => {$(
        /// Client errors keep the rejection text as message; server errors hide it as source.
        impl From<$rejection> for ApiError {
            fn from(rejection: $rejection) -> Self {
                let error = Self::new(rejection.status());
                if error.status.is_server_error() {
                    error.with_source(rejection)
                } else {
                    error.with_message(rejection.body_text())
                }
            }
        }
    )*};
}

from_rejection!(JsonRejection, QueryRejection, PathRejection);
