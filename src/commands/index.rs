//! [`CommandsIndex`] and related utilities
//!
//! [`CommandsIndex`]: ./struct.CommandsIndex.html

use std::collections::HashMap;

use super::Command;

/// The handler for all the commands
pub struct CommandsIndex<S> {
    commands: HashMap<String, Command<S>>,
    prefix: String,
}

impl<S> CommandsIndex<S> {
    /// Creates a new [`CommandsIndex`] and sets its prefix (see [module-level
    /// documentation](../mod.commands.html)
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

    /// Adds a new command to `self`
    ///
    /// The command is identified by the unique name `name` and set up by the argument `command`.
    /// `name` must not contain whitespaces so this returns the `String` name has been cast into if
    /// a whitespace has been found inside
    pub fn command<T: Into<String>>(&mut self, name: T, command: Command<S>) -> Result<(), String> {
        let name = name.into();
        if name.find(char::is_whitespace).is_some() {
            return Err(name);
        }

        self.commands.insert(name, command);
        Ok(())
    }
}

