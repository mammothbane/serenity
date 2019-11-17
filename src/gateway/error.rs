use tungstenite::protocol::CloseFrame;
use failure::Fail;

/// An error that occurred while attempting to deal with the gateway.
///
/// Note that - from a user standpoint - there should be no situation in which
/// you manually handle these.
#[derive(Clone, Debug, Fail)]
pub enum GatewayError {
    /// There was an error building a URL.
    #[fail(display = "Error building url")]
    BuildingUrl,

    /// The connection closed, potentially uncleanly.
    #[fail(display = "Connection closed")]
    Closed(Option<CloseFrame<'static>>),

    /// Expected a Hello during a handshake
    #[fail(display = "Expected a Hello")]
    ExpectedHello,

    /// When there was an error sending a heartbeat.
    #[fail(display = "Failed sending a heartbeat")]
    HeartbeatFailed,

    /// When invalid authentication (a bad token) was sent in the IDENTIFY.
    #[fail(display = "Sent invalid authentication")]
    InvalidAuthentication,

    /// Expected a Ready or an InvalidateSession
    #[fail(display = "Expected a valid Handshake")]
    InvalidHandshake,

    /// An indicator that an unknown opcode was received from the gateway.
    #[fail(display = "Invalid OpCode")]
    InvalidOpCode,

    /// When invalid sharding data was sent in the IDENTIFY.
    ///
    /// # Examples
    ///
    /// Sending a shard ID of 5 when sharding with 3 total is considered
    /// invalid.
    #[fail(display = "Sent invalid shard data")]
    InvalidShardData,

    /// When no authentication was sent in the IDENTIFY.
    #[fail(display = "Sent no authentication")]
    NoAuthentication,

    /// When a session Id was expected (for resuming), but was not present.
    #[fail(display = "No Session Id present when required")]
    NoSessionId,

    /// When a shard would have too many guilds assigned to it.
    ///
    /// # Examples
    ///
    /// When sharding 5500 guilds on 2 shards, at least one of the shards will
    /// have over the maximum number of allowed guilds per shard.
    ///
    /// This limit is currently 2500 guilds per shard.
    #[fail(display = "Shard has too many guilds")]
    OverloadedShard,

    /// Failed to reconnect after a number of attempts.
    #[fail(display = "Failed to Reconnect")]
    ReconnectFailure,
}
