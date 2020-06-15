//! # Panda Commands
//!
//! ## Introduction to commands
//! The commands API, used to easily create commands for bots. For example, a user could type
//! ```text
//! !hello
//! ```
//! to send the command `hello` to the bot, and the bot will do the specified work
//!
//! ## Syntax
//! A command is called like this
//! ```text
//! <prefix><command> <arguments>
//! ```
//! where `<prefix>` is any token, the same for all the commands, and used to recognize the bot
//! which shall run the given command. Different bots should have different prefixes, and it is
//! **not** recommended to use the widely used `!` prefix.
//!
//! `command` is the name of the command to run.
//!
//! `arguments` is some text passed to the command to work on it. Arguments are optional and
//! separated from `command` by a whitespace.

use std::sync::Arc;

use futures::future::BoxFuture;

use crate::models::channel::Message;
use crate::Session;


/// The built-in command type
pub struct Command<S> {
    callback: Box<CommandCallback<S>>,
}

impl<S> Command<S> {
    /// Creates a new [`Command`] from a callback
    ///
    /// [`Command`]: ./struct.Command.html
    pub fn new(callback: Box<CommandCallback<S>>) -> Self {
        Self {
            callback,
        }
    }
}

/// The type of the callback accepted by [`crate::commands::Command`]
type CommandCallback<S> = dyn Fn(Arc<Session<S>>, Message) -> BoxFuture<'static, CommandResult> + Send + Sync;

/// The result returned by a [`Command`] when run
///
/// [`Command`]: ../commands/struct.Command.html
pub type CommandResult = Result<(), Box<dyn std::error::Error>>;

