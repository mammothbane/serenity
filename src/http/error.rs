use std::convert::From;

use hyper::status::StatusCode;

use native_tls::Error as TlsError;
use serde_json::Error as JsonError;

#[derive(Debug, Fail)]
pub enum HttpError {
    /// When a non-successful status code was received for a request.
    #[fail(display = "A non-successful response status code was received: {}", status)]
    UnsuccessfulRequest {
        status: StatusCode,
    },

    /// When the decoding of a ratelimit header could not be properly decoded
    /// into an `i64`.
    #[fail(display = "Error decoding a header into an i64")]
    RateLimitI64,

    /// When the decoding of a ratelimit header could not be properly decoded
    /// from UTF-8.
    #[fail(display = "Error decoding a header from UTF-8")]
    RateLimitUtf8,

    /// An error from the `hyper` crate.
    #[fail(display = "Hyper error: {}", inner)]
    Hyper {
        inner: Box<::hyper::Error>,
    },

    #[fail(display = "Tls error: {}", inner)]
    Tls {
        inner: TlsError,
    },

    #[fail(display = "Json error: {}", inner)]
    Json {
        inner: JsonError,
    }
}

impl From<TlsError> for HttpError {
    fn from(inner: TlsError) -> Self {
        HttpError::Tls {
            inner
        }
    }
}

impl From<JsonError> for HttpError {
    fn from(inner: JsonError) -> Self {
        HttpError::Json {
            inner
        }
    }
}