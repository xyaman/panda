//! [`CommandsIndex`] and related utilities
//!
//! [`CommandsIndex`]: ./struct.CommandsIndex.html

use std::collections::HashMap;

use super::Command;

/// The handler for all the commands
///
/// Handles parsing and stores commands callbacks
///
/// # Example
/// ```
/// use std::sync::Arc;
///
/// use panda::client::SessionData;
/// use panda::commands::{Command, CommandResult, CommandsIndex};
/// use panda::models::channel::Message;
///
/// async fn ping(session: Arc<SessionData<()>>, msg: Message) -> CommandResult {
///     msg.send(&session.http, "Pong").await?;
///     Ok(())
/// }
///
/// // Defining a `CommandsIndex`. We tell to panda we need a bot using the `?` prefix and a `ping`
/// // command to run the function `ping`
/// let mut index = CommandsIndex::new("?");
/// index.command("ping", Command::new(ping)).unwrap();
///
/// // The user typed `!ping` (the prefix is wrong)
/// assert!(index.parse("!ping").is_none());
///
/// // The user typed `?pong` (the command name is wrong)
/// assert!(index.parse("?pong").is_none());
///
/// // The user typed `?ping` (good command invocation)
/// assert!(index.parse("?ping").is_some());
/// ```
///
/// The [module-level documentation] shows how to create a bot using commands
///
/// [module-level documentation]: ./index.html
pub struct CommandsIndex<S> {
    commands: HashMap<String, Command<S>>,
    prefix: String,
}

impl<S> CommandsIndex<S> {
    /// Creates a new [`CommandsIndex`] and sets its prefix (see [module-level
    /// documentation](./index.html))
    ///
    /// The newly created [`CommandsIndex`] instance is empty, meaning there isn't any command
    /// defined. If you start your bot with an empty [`CommandsIndex`], it won't react to any
    /// command
    ///
    /// [`CommandsIndex`]: ./struct.CommandsIndex.html
    pub fn new<T: Into<String>>(prefix: T) -> Self {
        Self {
            commands: HashMap::new(),
            prefix: prefix.into(),
        }
    }

    /// Adds a new [`Command`] to `self`
    ///
    /// The [`Command`] is identified by the unique name `name` and set up by the argument `command`.
    /// `name` must not contain whitespaces so this returns the `String` name has been cast into if
    /// a whitespace has been found inside
    ///
    /// [`Command`]: ./struct.Command.html
    pub fn command<T: Into<String>>(&mut self, name: T, command: Command<S>) -> Result<(), String> {
        let name = name.into();
        if name.find(char::is_whitespace).is_some() {
            return Err(name);
        }

        self.commands.insert(name, command);
        Ok(())
    }

    /// Parses a command sent by the user. Returns the [`Command`] to call or `None`
    ///
    /// [`Command`]: ./struct.Command.html
    pub fn parse(&self, command: &str) -> Option<&Command<S>> {
        if !command.starts_with(&self.prefix) {
            // There isn't anything to do
            return None;
        }
        let command = &command[self.prefix.len()..];

        let whitespace = command.find(char::is_whitespace).unwrap_or(command.len());
        let command_name = String::from(&command[..whitespace]);
        self.commands.get(&command_name)
    }
}

