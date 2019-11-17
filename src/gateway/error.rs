use tungstenite::protocol::CloseFrame;
use thiserror::Error;

/// An error that occurred while attempting to deal with the gateway.
///
/// Note that - from a user standpoint - there should be no situation in which
/// you manually handle these.
#[derive(Clone, Debug, Error)]
pub enum GatewayError {
    /// There was an error building a URL.
    #[error("Error building url")]
    BuildingUrl,

    /// The connection closed, potentially uncleanly.
    #[error("Connection closed")]
    Closed(Option<CloseFrame<'static>>),

    /// Expected a Hello during a handshake
    #[error("Expected a Hello")]
    ExpectedHello,

    /// When there was an error sending a heartbeat.
    #[error("Failed sending a heartbeat")]
    HeartbeatFailed,

    /// When invalid authentication (a bad token) was sent in the IDENTIFY.
    #[error("Sent invalid authentication")]
    InvalidAuthentication,

    /// Expected a Ready or an InvalidateSession
    #[error("Expected a valid Handshake")]
    InvalidHandshake,

    /// An indicator that an unknown opcode was received from the gateway.
    #[error("Invalid OpCode")]
    InvalidOpCode,

    /// When invalid sharding data was sent in the IDENTIFY.
    ///
    /// # Examples
    ///
    /// Sending a shard ID of 5 when sharding with 3 total is considered
    /// invalid.
    #[error("Sent invalid shard data")]
    InvalidShardData,

    /// When no authentication was sent in the IDENTIFY.
    #[error("Sent no authentication")]
    NoAuthentication,

    /// When a session Id was expected (for resuming), but was not present.
    #[error("No Session Id present when required")]
    NoSessionId,

    /// When a shard would have too many guilds assigned to it.
    ///
    /// # Examples
    ///
    /// When sharding 5500 guilds on 2 shards, at least one of the shards will
    /// have over the maximum number of allowed guilds per shard.
    ///
    /// This limit is currently 2500 guilds per shard.
    #[error("Shard has too many guilds")]
    OverloadedShard,

    /// Failed to reconnect after a number of attempts.
    #[error("Failed to Reconnect")]
    ReconnectFailure,
}
