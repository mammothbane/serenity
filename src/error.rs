use failure::Fail;

use crate::internal::prelude::*;

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
    ExceededLimit{ reason: String, limit: u32 },

    /// An error from the `url` crate.
    #[fail(display = "url error: {}", _0)]
    Url(String),

    /// Some other error. This is only used for "Expected value <TYPE>" errors,
    /// when a more detailed error can not be easily provided via the
    /// [`Error::Decode`] variant.
    ///
    /// [`Error::Decode`]: #variant.Decode
    #[fail(display = "other error: {}", _0)]
    Other(&'static str),
}
