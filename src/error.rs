use internal::prelude::*;
use serde_json::Error as JsonError;

/// Common errors returned through the library.
///
/// [`Result`]: type.Result.html
#[derive(Debug, Fail)]
pub enum SerenityError {
    /// An error while decoding a payload.
    #[fail(display = "{}", msg)]
    Decode {
        msg: &'static str,
        value: Value,
    },

    /// Input exceeded a limit.
    /// Providing the input and the limit that's not supposed to be exceeded.
    ///
    /// *This only exists for the `GuildId::ban` and `Member::ban` functions. For their cases,
    /// it's the "reason".*
    #[fail(display = "Input exceeded a limit")]
    ExceededLimit {
        reason: String,
        limit: u32,
    },

    /// An error from the `serde_json` crate.
    #[fail(display = "JSON error: {}", wrapped)]
    Json {
        wrapped: JsonError,
    },
}
