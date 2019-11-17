use std::process::Output;

use serde_json::{Error as JsonError, Value};
use opus::Error as OpusError;

/// An error returned from the voice module.
#[derive(Debug, Fail)]
pub enum VoiceError {
    // Errors which are not visible to the end user are hidden.
    #[fail(display = "expected handshake")]
    #[doc(hidden)] ExpectedHandshake,

    #[fail(display = "couldn't find desired byte")]
    #[doc(hidden)] FindingByte,

    #[fail(display = "resolving hostname")]
    #[doc(hidden)] HostnameResolve,

    #[fail(display = "generating key")]
    #[doc(hidden)] KeyGen,

    #[fail(display = "invalid voice mode")]
    #[doc(hidden)] VoiceModeInvalid,

    #[fail(display = "selected voice mode unavailable")]
    #[doc(hidden)] VoiceModeUnavailable,

    /// An indicator that an endpoint URL was invalid.
    #[fail(display = "endpoint url was invalid")]
    EndpointUrl,

    /// An error occurred while checking if a path is stereo.
    #[fail(display = "checking if a path was stereo")]
    Streams,

    /// An error occurred while running `youtube-dl`.
    #[fail(display = "failed to run youtube-dl")]
    YouTubeDLRun(Output),

    /// An error occurred while processing the JSON output from `youtube-dl`.
    ///
    /// The JSON output is given.
    #[fail(display = "processing json from youtube-dl")]
    YouTubeDLProcessing(Value),

    /// The `url` field of the `youtube-dl` JSON output was not present.
    ///
    /// The JSON output is given.
    #[fail(display = "url field missing from youtube-dl output")]
    YouTubeDLUrl(Value),
}

/// An error returned from the dca method.
#[derive(Debug, Fail)]
pub enum DcaError {
    #[fail(display = "invalid DCA header")]
    InvalidHeader,

    #[fail(display = "{}", _0)]
    InvalidMetadata(#[cause] JsonError),

    #[fail(display = "invalid DCA size")]
    InvalidSize(i32),

    #[doc(hidden)]
    #[fail(display = "unreachable")]
    __Nonexhaustive,
}
