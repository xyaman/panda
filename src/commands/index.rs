//! [`CommandsIndex`] and related utilities
//!
//! [`CommandsIndex`]: ./struct.CommandsIndex.html

use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

use crate::client::SessionData;
use crate::models::events::MessageCreate;
use super::Command;

/// The handler for all the commands. Handles parsing and stores commands callbacks
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

    /// Parses a command sent by the user. Returns the [`Command`] to call with its arguments or
    /// `None`
    ///
    /// [`Command`]: ./struct.Command.html
    fn parse(&self, command: &str) -> Option<(&Command<S>, String)> {
        if !command.starts_with(&self.prefix) {
            // There isn't anything to do
            return None;
        }
        let command = &command[self.prefix.len()..];

        let whitespace = command.find(char::is_whitespace).unwrap_or(command.len());
        let command_name = String::from(&command[..whitespace]);
        let args = if whitespace + 1 >= command.len() {
            String::new()
        } else {
            String::from(&command[whitespace + 1..])
        };

        let command = self.commands.get(&command_name)?;
        Some((command, args))
    }
}

/// A shorthand function to easily create a handler to plug a [`CommandsIndex`] in a [`Client`]
///
/// # Usage
/// Don't use it directly but use it through the macro [`make_commands_handler`]
///
/// [`CommandsIndex`]: ./struct.CommandsIndex.html
/// [`make_commands_handler`]: ../macro.make_commands_handler.html
pub async fn handle_commands<S>(index: Arc<CommandsIndex<S>>, session: Arc<SessionData<S>>, event: MessageCreate) -> Result<(), Box<dyn Error>> {
    let (command, args) = match index.parse(&event.content) {
        Some(val) => val,
        // There isn't any command invocation
        None      => return Ok(()),
    };

    command.run(session, event.0, args).await
}

/// Generates a closure to automate the parsing of commands sent to the bot
#[macro_export]
macro_rules! make_commands_handler(
    ($index: expr) => ({
        let index = std::sync::Arc::new($index);

        move |session, event| {
            $crate::commands::handle_commands(index.clone(), session, event)
        }
    })
);

