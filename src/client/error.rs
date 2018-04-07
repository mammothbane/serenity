/// An error returned from the [`Client`].
///
/// [`Client`]: struct.Client.html
#[derive(Clone, Debug, Eq, Hash, PartialEq, Fail)]
pub enum ClientError {
    /// When the token provided is invalid. This is returned when validating a
    /// token through the [`validate_token`] function.
    ///
    /// [`validate_token`]: fn.validate_token.html
    #[fail(display = "The provided token was invalid")]
    InvalidToken,
    /// When a shard has completely failed to reboot after resume and/or
    /// reconnect attempts.
    #[fail(display = "Failed to (re-)boot a shard")]
    ShardBootFailure,
    /// When all shards that the client is responsible for have shutdown with an
    /// error.
    #[fail(display = "The client's shard shut down")]
    Shutdown,
}
