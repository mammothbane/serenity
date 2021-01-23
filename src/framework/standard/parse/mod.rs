use std::borrow::Cow;

use thiserror::Error;
use uwl::{StrExt, UnicodeStream};

use crate::client::Context;
use crate::model::channel::Message;

use super::*;

use self::map::{CommandMap, GroupMap, ParseMap};

pub mod map;


#[inline]
fn to_lowercase<'a>(config: &Configuration, s: &'a str) -> Cow<'a, str> {
    if config.case_insensitive {
        Cow::Owned(s.to_lowercase())
    } else {
        Cow::Borrowed(s)
    }
}

/// Parse a mention in the message that is of either the direct (`<@id>`) or nickname (`<@!id>`) syntax,
/// and compare the encoded `id` with the id from [`Configuration::on_mention`] for a match.
/// Returns `Some(<id>)` on success, `None` otherwise.
///
/// [`Configuration::on_mention`]: ../struct.Configuration.html#method.on_mention
pub fn mention<'a>(stream: &mut UnicodeStream<'a>, config: &Configuration) -> Option<&'a str> {
    let on_mention = config.on_mention.as_ref().map(String::as_str)?;

    let start = stream.offset();

    if !stream.eat("<@") {
        return None;
    }

    // Optional.
    stream.eat("!");

    let id = stream.take_while(|s| s.is_numeric());

    if !stream.eat(">") {
        // Backtrack to where we were.
        stream.set(start);

        return None;
    }

    if id == on_mention {
        Some(id)
    } else {
        stream.set(start);

        None
    }
}

fn find_prefix<'a>(
    ctx: &mut Context,
    msg: &Message,
    config: &Configuration,
    stream: &UnicodeStream<'a>,
) -> Option<&'a str> {
    let try_match = |prefix: &str| {
        let peeked = stream.peek_for(prefix.chars().count());

        if prefix == peeked {
            Some(peeked)
        } else {
            None
        }
    };

    for f in &config.dynamic_prefixes {
        if let Some(p) = f(ctx, msg) {
            if let Some(p) = try_match(&p) {
                return Some(p);
            }
        }
    }

    config.prefixes.iter().find_map(|p| try_match(&p))
}

/// Parse a prefix in the message.
///
/// The "prefix" may be one of the following:
/// - A mention (`<@id>`/`<@!id>`)
/// - A dynamically constructed prefix ([`Configuration::dynamic_prefix`])
/// - A static prefix ([`Configuration::prefix`])
/// - Nothing
///
/// In all cases, whitespace after the prefix is cleared.
///
/// [`Configuration::dynamic_prefix`]: ../struct.Configuration.html#method.dynamic_prefix
/// [`Configuration::prefix`]: ../struct.Configuration.html#method.prefix
pub fn prefix<'a>(
    ctx: &mut Context,
    msg: &Message,
    stream: &mut UnicodeStream<'a>,
    config: &Configuration,
) -> Option<&'a str> {
    if let Some(id) = mention(stream, config) {
        stream.take_while(|s| s.is_whitespace());

        return Some(id);
    }

    let prefix = find_prefix(ctx, msg, config, stream);

    if let Some(prefix) = prefix {
        stream.increment(prefix.len());
    }

    if config.with_whitespace.prefixes {
        stream.take_while(|s| s.is_whitespace());
    }

    prefix
}

/// Checked per valid group or command in the message.
fn check_discrepancy(
    ctx: &Context,
    msg: &Message,
    config: &Configuration,
    options: &impl CommonOptions,
) -> StdResult<(), DispatchError> {
    if options.owners_only() && !config.owners.contains(&msg.author.id) {
        return Err(DispatchError::OnlyForOwners);
    }

    if options.only_in() == OnlyIn::Dm && !msg.is_private() {
        return Err(DispatchError::OnlyForDM);
    }

    if (!config.allow_dm || options.only_in() == OnlyIn::Guild) && msg.is_private() {
        return Err(DispatchError::OnlyForGuilds);
    }

    #[cfg(feature = "cache")]
    {
        if let Some(guild_id) = msg.guild_id {
            let guild = match guild_id.to_guild_cached(&ctx) {
                Some(g) => g,
                None => return Ok(()),
            };

            let guild = guild.read();

            let perms = guild.user_permissions_in(msg.channel_id, msg.author.id);

            if !perms.contains(*options.required_permissions())
                && !(options.owner_privilege() && config.owners.contains(&msg.author.id))
            {
                return Err(DispatchError::LackingPermissions(
                    *options.required_permissions(),
                ));
            }

            if let Some(member) = guild.members.get(&msg.author.id) {
                if !perms.administrator() && !has_correct_roles(options, &guild, &member) {
                    return Err(DispatchError::LackingRole);
                }
            }
        }
    }

    Ok(())
}

fn try_parse<M: ParseMap>(
    stream: &mut UnicodeStream<'_>,
    map: &M,
    by_space: bool,
    f: impl Fn(&str) -> String,
) -> (String, Option<M::Storage>) {
    if by_space {
        let n = f(stream.peek_until(|s| s.is_whitespace()));

        let o = map.get(&n);

        (n, o)
    } else {
        let mut n = f(stream.peek_for(map.max_length()));
        let mut o = None;

        for _ in 0..(map.max_length() - map.min_length()) {
            o = map.get(&n);

            if o.is_some() {
                break;
            }

            n.pop();
        }

        (n, o)
    }
}

fn parse_cmd(
    stream: &mut UnicodeStream<'_>,
    ctx: &Context,
    msg: &Message,
    config: &Configuration,
    map: &CommandMap,
) -> Result<&'static Command> {
    let (n, r) = try_parse(stream, map, config.by_space, |s| {
        to_lowercase(config, s).into_owned()
    });

    if config.disabled_commands.contains(&n) {
        return Err(DispatchError::CommandDisabled(n).into());
    }

    if let Some((cmd, map)) = r {
        stream.increment(n.len());

        if config.with_whitespace.commands {
            stream.take_while(|s| s.is_whitespace());
        }

        check_discrepancy(ctx, msg, config, &cmd.options)?;

        if map.is_empty() {
            return Ok(cmd);
        }

        return match parse_cmd(stream, ctx, msg, config, &map) {
            Err(e) => match e.downcast()? {
                UnknownCommand(Some(_)) => Ok(cmd),
                e => Err(e.into()),
            },
            res => res,
        };
    }

    Err(UnknownCommand(Some(n.to_string())).into())
}

fn parse_group(
    stream: &mut UnicodeStream<'_>,
    ctx: &Context,
    msg: &Message,
    config: &Configuration,
    map: &GroupMap,
) -> Result<(&'static CommandGroup, Arc<CommandMap>)> {
    let (n, o) = try_parse(stream, map, config.by_space, ToString::to_string);

    if let Some((group, map, commands)) = o {
        stream.increment(n.len());

        if config.with_whitespace.groups {
            stream.take_while(|s| s.is_whitespace());
        }

        check_discrepancy(ctx, msg, config, &group.options)?;

        if map.is_empty() {
            return Ok((group, commands));
        }

        return match parse_group(stream, ctx, msg, config, &map) {
            Err(e) => match e.downcast()? {
                UnknownCommand(None) => Ok((group, commands)),
                res => Err(res.into()),
            }
            res => res,
        };
    }

    Err(UnknownCommand(None).into())
}

#[inline]
fn handle_command(
    stream: &mut UnicodeStream<'_>,
    ctx: &Context,
    msg: &Message,
    config: &Configuration,
    map: &CommandMap,
    group: &'static CommandGroup,
) -> Result<Invoke> {
    match parse_cmd(stream, ctx, msg, config, map) {
        Ok(command) => Ok(Invoke::Command { group, command }),
        Err(err) => match group.options.default_command {
            Some(command) => Ok(Invoke::Command { group, command }),
            None => Err(err.into()),
        },
    }
}

#[inline]
fn handle_group(
    stream: &mut UnicodeStream<'_>,
    ctx: &Context,
    msg: &Message,
    config: &Configuration,
    map: &GroupMap,
) -> Result<Invoke> {
    parse_group(stream, ctx, msg, config, map)
        .and_then(|(group, map)| handle_command(stream, ctx, msg, config, &map, group))
}

#[derive(Debug, Clone, Error, PartialEq, Eq, Hash)]
#[error("parse failed, context: {:?}", _0)]
pub struct UnknownCommand(pub Option<String>);

/// Parse a command from the message.
///
/// The "command" may be:
/// 1. A *help command* that provides a friendly browsing interface of all groups and commands,
/// explaining what each of them are, how they are layed out and how to invoke them.
/// There can only one help command registered, but might have many names defined for invocation of itself.
///
/// 2. A command defined under another command or a group, which may also belong to another group and so on.
/// To invoke this command, all names and prefixes of its parent commands and groups must be specified before it.
pub fn command(
    ctx: &Context,
    msg: &Message,
    stream: &mut UnicodeStream<'_>,
    groups: &[(&'static CommandGroup, Map)],
    config: &Configuration,
    help_was_set: Option<&[&'static str]>,
) -> Result<Invoke> {
    // Precedence is taken over commands named as one of the help names.
    if let Some(names) = help_was_set {
        for name in names {
            let n = to_lowercase(config, stream.peek_for(name.chars().count()));

            if name == &n {
                stream.increment(n.len());

                stream.take_while(|s| s.is_whitespace());

                return Ok(Invoke::Help(name));
            }
        }
    }

    let mut last = Err(UnknownCommand(None).into());

    for (group, map) in groups {
        match map {
            // Includes [group] itself.
            Map::WithPrefixes(map) => {
                let res = handle_group(stream, ctx, msg, config, map);

                if res.is_ok() {
                    return res;
                }

                last = res;
            }
            Map::Prefixless(subgroups, commands) => {
                let res = handle_group(stream, ctx, msg, config, subgroups);

                if res.is_ok() {
                    check_discrepancy(ctx, msg, config, &group.options)?;

                    return res;
                }

                let res = handle_command(stream, ctx, msg, config, commands, group);

                if res.is_ok() {
                    check_discrepancy(ctx, msg, config, &group.options)?;

                    return res;
                }

                last = res;
            }
        }
    }

    last
}

#[derive(Debug)]
pub enum Invoke {
    Command {
        group: &'static CommandGroup,
        command: &'static Command,
    },
    Help(&'static str),
}