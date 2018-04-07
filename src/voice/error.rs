use serde_json::{Error as JsonError, Value};
use std::process::Output;

/// An error returned from the voice module.
// Errors which are not visible to the end user are hidden.
#[derive(Debug, Fail)]
pub enum VoiceError {
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

    /// An error from the `opus` crate.
    #[fail(display = "Opus error: {}", wrapped)]
    Opus {
        wrapped: OpusError,
    },
}

/// An error returned from the dca method.
#[derive(Debug, Fail)]
pub enum DcaError {
    #[fail(display = "invalid DCA header")]
    InvalidHeader,

    #[fail(display = "invalid metadata: {}", wrapped)]
    InvalidMetadata {
        wrapped: JsonError,
    },

    #[fail(display = "invalid DCA size")]
    InvalidSize(i32),
}
