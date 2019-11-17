use reqwest::{
    header::InvalidHeaderValue,
    Response,
    StatusCode,
    Url,
};

use thiserror::Error;

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct DiscordJsonError {
    pub code: isize,
    pub message: String,
    #[serde(skip)]
    non_exhaustive: (),
}

impl std::fmt::Debug for DiscordJsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.message)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ErrorResponse {
    pub status_code: StatusCode,
    pub url: Url,
    pub error: DiscordJsonError,
}

impl From<Response> for ErrorResponse {
    fn from(mut r: Response) -> Self {
        ErrorResponse {
            status_code: r.status(),
            url: r.url().clone(),
            error: r.json().unwrap_or_else(|_| DiscordJsonError {
                code: -1,
                message: "[Serenity] No correct json was received!".to_string(),
                non_exhaustive: (),
            }),
        }
    }
}


#[derive(Debug, Error)]
pub enum HttpError {
    /// When a non-successful status code was received for a request.
    #[error("request failed: {:?}", _0)]
    UnsuccessfulRequest(ErrorResponse),

    /// When the decoding of a ratelimit header could not be properly decoded
    /// into an `i64`.
    #[error("Error decoding a header into an i64")]
    RateLimitI64,

    /// When the decoding of a ratelimit header could not be properly decoded
    /// from UTF-8.
    #[error("Error decoding a header from UTF-8")]
    RateLimitUtf8,

    /// Header value contains invalid input.
    #[error("invalid header value")]
    InvalidHeader(InvalidHeaderValue),
}

impl From<InvalidHeaderValue> for HttpError {
    fn from(error: InvalidHeaderValue) -> HttpError {
        HttpError::InvalidHeader(error)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use http_crate::response::Builder;
    use reqwest::r#async::ResponseBuilderExt;

    #[test]
    fn test_error_response_into() {
        let error = DiscordJsonError {
            code: 43121215,
            message: String::from("This is a Ferris error"),
            non_exhaustive: (),
        };

        let mut builder = Builder::new();
        builder.status(403);
        builder.url(String::from("https://ferris.crab").parse().unwrap());
        let body_string = serde_json::to_string(&error).unwrap();
        let response = builder.body(body_string.into_bytes()).unwrap();

        let reqwest_response: reqwest::Response = response.into();
        let error_response: ErrorResponse = reqwest_response.into();

        let known = ErrorResponse {
            status_code: reqwest::StatusCode::from_u16(403).unwrap(),
            url: String::from("https://ferris.crab").parse().unwrap(),
            error,
        };

        assert_eq!(error_response, known);
    }
}
