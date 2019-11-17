use failure::Fail;

#[cfg(feature = "http")]
use reqwest::{Error as ReqwestError, header::InvalidHeaderValue};

#[cfg(feature = "voice")]
use audiopus::Error as OpusError;

#[cfg(feature = "gateway")]
use tungstenite::error::Error as TungsteniteError;

use crate::model::ModelError;

#[cfg(feature = "client")]
use crate::client::ClientError;
#[cfg(feature = "gateway")]
use crate::gateway::GatewayError;
#[cfg(feature = "http")]
use crate::http::HttpError;
#[cfg(all(feature = "gateway", not(feature = "native_tls_backend")))]
use crate::internal::ws_impl::RustlsError;
#[cfg(feature = "voice")]
use crate::voice::VoiceError;

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

    /// Some other error. This is only used for "Expected value <TYPE>" errors,
    /// when a more detailed error can not be easily provided via the
    /// [`Error::Decode`] variant.
    ///
    /// [`Error::Decode`]: #variant.Decode
    #[fail(display = "other error: {}", _0)]
    Other(&'static str),

    /// An error from the `url` crate.
    #[fail(display = "url error: {}", _0)]
    Url(String),

    /// A [client] error.
    ///
    /// [client]: client/index.html
    #[cfg(feature = "client")]
    #[fail(display = "client error")]
    Client(#[cause] ClientError),

    /// An error from the `gateway` module.
    #[cfg(feature = "gateway")]
    #[fail(display = "gateway error")]
    Gateway(GatewayError),

    /// An error from the `model` module.
    #[fail(display = "model error")]
    Model(ModelError),

    /// An error from the [`http`] module.
    ///
    /// [`http`]: http/index.html
    #[cfg(feature = "http")]
    #[fail(display = "http error")]
    Http(Box<HttpError>),

    /// An error occuring in rustls
    #[cfg(all(feature = "gateway", not(feature = "native_tls_backend")))]
    #[fail(display = "rustls error")]
    Rustls(RustlsError),

    /// An error from the `tungstenite` crate.
    #[cfg(feature = "gateway")]
    #[fail(display = "tungstenite error")]
    Tungstenite(TungsteniteError),

    /// An error from the `opus` crate.
    #[cfg(feature = "voice")]
    #[fail(display = "opus error")]
    Opus(OpusError),

    /// Indicating an error within the [voice module].
    ///
    /// [voice module]: voice/index.html
    #[cfg(feature = "voice")]
    #[fail(display = "voice error")]
    Voice(VoiceError),
}

#[cfg(feature = "gateway")]
impl From<GatewayError> for SerenityError {
    fn from(e: GatewayError) -> SerenityError { SerenityError::Gateway(e) }
}

#[cfg(feature = "voice")]
impl From<OpusError> for SerenityError {
    fn from(e: OpusError) -> SerenityError { SerenityError::Opus(e) }
}

#[cfg(feature = "voice")]
impl From<VoiceError> for SerenityError {
    fn from(e: VoiceError) -> SerenityError { SerenityError::Voice(e) }
}

#[cfg(feature = "gateway")]
impl From<TungsteniteError> for SerenityError {
    fn from(e: TungsteniteError) -> SerenityError { SerenityError::Tungstenite(e) }
}

#[cfg(feature = "http")]
impl From<HttpError> for SerenityError {
    fn from(e: HttpError) -> SerenityError { SerenityError::Http(Box::new(e)) }
}

#[cfg(feature = "http")]
impl From<InvalidHeaderValue> for SerenityError {
    fn from(e: InvalidHeaderValue) -> SerenityError { HttpError::InvalidHeader(e).into() }
}

#[cfg(feature = "http")]
impl From<ReqwestError> for SerenityError {
    fn from(e: ReqwestError) -> SerenityError { HttpError::Request(e).into() }
}
