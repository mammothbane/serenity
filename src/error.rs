use crate::internal::prelude::*;
use crate::model::ModelError;
use serde_json::Error as JsonError;
use std::{
    error::Error as StdError,
    fmt::{
        self,
        Display,
        Error as FormatError
    },
    io::Error as IoError,
    num::ParseIntError
};

#[cfg(feature = "http")]
use reqwest::{Error as ReqwestError, header::InvalidHeaderValue};
#[cfg(feature = "voice")]
use audiopus::Error as OpusError;
#[cfg(feature = "gateway")]
use tungstenite::error::Error as TungsteniteError;
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
    ExceededLimit(String, u32),
    /// Some other error. This is only used for "Expected value <TYPE>" errors,
    /// when a more detailed error can not be easily provided via the
    /// [`Error::Decode`] variant.
    ///
    /// [`Error::Decode`]: #variant.Decode
    Other(&'static str),
    /// An error from the `url` crate.
    Url(String),
    /// A [client] error.
    ///
    /// [client]: client/index.html
    #[cfg(feature = "client")]
    Client(ClientError),
    /// An error from the `gateway` module.
    #[cfg(feature = "gateway")]
    Gateway(GatewayError),
    /// An error from the [`http`] module.
    ///
    /// [`http`]: http/index.html
    #[cfg(feature = "http")]
    Http(Box<HttpError>),
    /// An error occuring in rustls
    #[cfg(all(feature = "gateway", not(feature = "native_tls_backend")))]
    Rustls(RustlsError),
    /// An error from the `tungstenite` crate.
    #[cfg(feature = "gateway")]
    Tungstenite(TungsteniteError),
    /// An error from the `opus` crate.
    #[cfg(feature = "voice")]
    Opus(OpusError),
    /// Indicating an error within the [voice module].
    ///
    /// [voice module]: voice/index.html
    #[cfg(feature = "voice")]
    Voice(VoiceError),
    #[doc(hidden)]
    __Nonexhaustive,
}

impl From<FormatError> for Error {
    fn from(e: FormatError) -> Error { Error::Format(e) }
}

#[cfg(feature = "gateway")]
impl From<GatewayError> for Error {
    fn from(e: GatewayError) -> Error { Error::Gateway(e) }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Error { Error::Io(e) }
}

impl From<JsonError> for Error {
    fn from(e: JsonError) -> Error { Error::Json(e) }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Error { Error::Num(e) }
}

impl From<ModelError> for Error {
    fn from(e: ModelError) -> Error { Error::Model(e) }
}

#[cfg(feature = "voice")]
impl From<OpusError> for Error {
    fn from(e: OpusError) -> Error { Error::Opus(e) }
}

#[cfg(feature = "voice")]
impl From<VoiceError> for Error {
    fn from(e: VoiceError) -> Error { Error::Voice(e) }
}

#[cfg(all(feature = "gateway", not(feature = "native_tls_backend")))]
impl From<RustlsError> for Error {
    fn from(e: RustlsError) -> Error { Error::Rustls(e) }
}

#[cfg(feature = "gateway")]
impl From<TungsteniteError> for Error {
    fn from(e: TungsteniteError) -> Error { Error::Tungstenite(e) }
}

#[cfg(feature = "http")]
impl From<HttpError> for Error {
    fn from(e: HttpError) -> Error { Error::Http(Box::new(e)) }
}

#[cfg(feature = "http")]
impl From<InvalidHeaderValue> for Error {
    fn from(e: InvalidHeaderValue) -> Error { HttpError::InvalidHeader(e).into() }
}

#[cfg(feature = "http")]
impl From<ReqwestError> for Error {
    fn from(e: ReqwestError) -> Error { HttpError::Request(e).into() }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Decode(msg, _) | Error::Other(msg) => msg,
            Error::ExceededLimit(..) => "Input exceeded a limit",
            Error::Format(ref inner) => inner.description(),
            Error::Io(ref inner) => inner.description(),
            Error::Json(ref inner) => inner.description(),
            Error::Model(ref inner) => inner.description(),
            Error::Num(ref inner) => inner.description(),
            Error::Url(ref inner) => inner,
            #[cfg(feature = "client")]
            Error::Client(ref inner) => inner.description(),
            #[cfg(feature = "gateway")]
            Error::Gateway(ref inner) => inner.description(),
            #[cfg(feature = "http")]
            Error::Http(ref inner) => inner.description(),
            #[cfg(feature = "http")]
            Error::Hyper(ref inner) => inner.description(),
            #[cfg(feature = "voice")]
            Error::Opus(ref inner) => inner.description(),
            #[cfg(feature = "native-tls")]
            Error::Tls(ref inner) => inner.description(),
            #[cfg(feature = "voice")]
            Error::Voice(_) => "Voice error",
            #[cfg(feature = "gateway")]
            Error::WebSocket(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            #[cfg(feature = "http")]
            Error::Hyper(ref inner) => Some(inner),
            Error::Json(ref inner) => Some(inner),
            Error::Io(ref inner) => Some(inner),
            #[cfg(feature = "gateway")]
            Error::WebSocket(ref inner) => Some(inner),
            _ => None,
        }
    }
}
