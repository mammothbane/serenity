use std::process::Output;

use thiserror::Error;
use serde_json::Value;

/// An error returned from the voice module.
#[derive(Debug, Error)]
pub enum VoiceError {
    // Errors which are not visible to the end user are hidden.
    #[error("expected handshake")]
    #[doc(hidden)] ExpectedHandshake,

    #[error("couldn't find desired byte")]
    #[doc(hidden)] FindingByte,

    #[error("resolving hostname")]
    #[doc(hidden)] HostnameResolve,

    #[error("generating key")]
    #[doc(hidden)] KeyGen,

    #[error("invalid voice mode")]
    #[doc(hidden)] VoiceModeInvalid,

    #[error("selected voice mode unavailable")]
    #[doc(hidden)] VoiceModeUnavailable,

    /// An indicator that an endpoint URL was invalid.
    #[error("endpoint url was invalid")]
    EndpointUrl,

    /// An error occurred while checking if a path is stereo.
    #[error("checking if a path was stereo")]
    Streams,

    /// An error occurred while running `youtube-dl`.
    #[error("failed to run youtube-dl")]
    YouTubeDLRun(Output),

    /// An error occurred while processing the JSON output from `youtube-dl`.
    ///
    /// The JSON output is given.
    #[error("processing json from youtube-dl")]
    YouTubeDLProcessing(Value),

    /// The `url` field of the `youtube-dl` JSON output was not present.
    ///
    /// The JSON output is given.
    #[error("url field missing from youtube-dl output")]
    YouTubeDLUrl(Value),
}

/// An error returned from the dca method.
#[derive(Debug, Error)]
pub enum DcaError {
    #[error("invalid DCA header")]
    InvalidHeader,

    #[error("invalid DCA size")]
    InvalidSize(i32),

    #[doc(hidden)]
    #[error("unreachable")]
    __Nonexhaustive,
}
