//! Error enum definition wrapping potential model implementation errors.

use failure::Fail;

use super::Permissions;

/// An error returned from the [`model`] module.
///
/// # Examples
///
/// Matching an [`Error`] with this variant would look something like the
/// following for the [`GuildId::ban`] method, which in this example is used to
/// re-ban all members with an odd discriminator:
///
/// ```rust,no_run
/// # #[cfg(all(feature = "client", feature = "model"))]
/// # use std::error::Error;
/// #
/// # #[cfg(all(feature = "client", feature = "model"))]
/// # fn try_main() -> Result<(), Box<Error>> {
/// use serenity::prelude::*;
/// use serenity::model::prelude::*;
/// use serenity::model::ModelError;
/// use std::env;
///
/// struct Handler;
///
/// impl EventHandler for Handler {
///     fn guild_ban_removal(&self, context: Context, guild_id: GuildId, user: User) {
///         // If the user has an even discriminator, don't re-ban them.
///         if user.discriminator % 2 == 0 {
///             return;
///         }
///
///         match guild_id.ban(&context.http, user, &8) {
///             Ok(()) => {
///                 // Ban successful.
///             },
///             Err(ModelError::DeleteMessageDaysAmount(amount)) => {
///                 println!("Failed deleting {} days' worth of messages", amount);
///             },
///             Err(why) => {
///                 println!("Unexpected error: {:?}", why);
///             },
///         }
///     }
/// }
/// let token = env::var("DISCORD_BOT_TOKEN")?;
/// let mut client = Client::new(&token, Handler).unwrap();
///
/// client.start()?;
/// #     Ok(())
/// # }
/// #
/// # #[cfg(all(feature = "client", feature = "model"))]
/// # fn main() {
/// #     try_main().unwrap();
/// # }
/// #
/// # #[cfg(not(all(feature="client", feature = "model")))]
/// # fn main() { }
/// ```
///
/// [`Error`]: ../enum.Error.html
/// [`GuildId::ban`]: struct.GuildId.html#method.ban
/// [`model`]: ./index.html
#[derive(Clone, Debug, Eq, Hash, PartialEq, Fail)]
pub enum ModelError {
    /// Attempting to delete outside the permissible range of bulk message deletions.
    #[fail(display = "Too few/many messages to bulk delete")]
    BulkDeleteAmount,

    /// Attempting to delete a disallowed number of days of messages.
    #[fail(display = "Invalid delete message days")]
    DeleteMessageDaysAmount(u8),

    /// The textual content of an embed exceeds the maximum length.
    #[fail(display = "Embed too large")]
    EmbedTooLarge(u64),

    /// A [guild][`Guild`] could not be found by the given
    /// [Id][`GuildId`] in the [`Cache`].
    ///
    /// [`Guild`]: ../guild/struct.Guild.html
    /// [`GuildId`]: ../id/struct.GuildId.html
    /// [`Cache`]: ../../cache/struct.Cache.html
    #[fail(display = "Guild not found in the cache")]
    GuildNotFound,

    /// An indication that a [role][`Role`] could not be found by
    /// [Id][`RoleId`] in the [`Cache`].
    ///
    /// [`Role`]: ../guild/struct.Role.html
    /// [`RoleId`]: ../id/struct.GuildId.html
    /// [`Cache`]: ../../cache/struct.Cache.html
    #[fail(display = "role couldn't be found by id in cache")]
    RoleNotFound,

    /// Indicates that there are hierarchy problems restricting an action.
    ///
    /// For example, when banning a user, if the other user has a role with an
    /// equal to or higher position, then they can not be banned.
    ///
    /// When editing a role, if the role is higher in position than the current
    /// user's highest role, then the role can not be edited.
    #[fail(display = "Role hierarchy prevents this action")]
    Hierarchy,

    /// The current user does not have the required permissions to perform an
    /// operation.
    ///
    /// The provided [`Permission`]s is the set of permissions required.
    ///
    /// [`Permission`]: ../permissions/struct.Permissions.html
    #[fail(display = "Invalid permissions")]
    InvalidPermissions(Permissions),

    /// The [current user] could not perform an action.
    ///
    /// [current user]: ../user/struct.CurrentUser.html
    #[fail(display = "The current user can not perform the action")]
    InvalidUser,

    /// The item is missing from the [`Cache`], so the action could not continue.
    ///
    /// [`Cache`]: ../../cache/struct.Cache.html
    #[fail(display = "The required item is missing from the cache")]
    ItemMissing,

    /// A [`Message`]s content was longer than that maximum 2000 codepoints or 4000 bytes.
    ///
    /// The number of bytes overflowed is included.
    ///
    /// [`Message`]: ../channel/struct.Message.html
    #[fail(display = "Message too large")]
    MessageTooLong(u64),

    /// The current user is attempting to Direct Message another
    /// bot user, which is disallowed by the API.
    #[fail(display = "Attempted to message another bot user")]
    MessagingBot,

    /// An indicator that the [`ChannelType`] cannot perform an action.
    ///
    /// [`ChannelType`]: ../channel/enum.ChannelType.html
    #[fail(display = "invalid channel type")]
    InvalidChannelType,
}
