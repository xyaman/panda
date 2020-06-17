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
//!
//! # Example
//! ```rust,should_panic
//! use std::sync::Arc;
//!
//! use panda::client::SessionData;
//! use panda::commands::{Command, CommandResult, CommandsIndex};
//! use panda::make_commands_handler;
//! use panda::models::channel::Message;
//!
//! async fn ping(session: Arc<SessionData<()>>, msg: Message, _args: String) -> CommandResult {
//!     msg.send(&session.http, "Pong").await?;
//!     Ok(())
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // We create a new CommandsIndex to tell panda we need a bot using the prefix `?` and the
//!     // defining the command `ping` which calls the async function `ping`
//!     let mut index = CommandsIndex::new("?");
//!     index.command("ping", Command::new(ping)).unwrap();
//!
//!     // And then, we create the bot and an event handler for the event `MessageCreate` which
//!     // runs the requested commands
//!     let mut bot = panda::new("your token here").await.unwrap();
//!     bot.on_message_create(make_commands_handler!(index));
//!
//!     // The last step is to start the newly created bot !
//!     bot.start().await.unwrap();
//!
//!     Ok(())
//! }
//! ```

use std::future::Future;
use std::sync::Arc;

use futures::future::BoxFuture;

use crate::models::channel::Message;
use crate::client::SessionData;


/// The built-in command type
pub struct Command<S> {
    callback: Box<CommandCallback<S>>,
}

impl<S> Command<S> {
    /// Creates a new [`Command`] from a callback
    ///
    /// [`Command`]: ./struct.Command.html
    pub fn new<F, Fut>(callback: F) -> Self
    where
        F: Send + Sync + 'static,
        F: Fn(Arc<SessionData<S>>, Message, String) -> Fut,
        Fut: Send + 'static,
        Fut: Future<Output = CommandResult>,
    {
        let pinned = move |session, msg, args| -> BoxFuture<'static, CommandResult> {
            Box::pin(callback(session, msg, args))
        };

        Self {
            callback: Box::new(pinned),
        }
    }

    /// Runs `self`
    pub async fn run(&self, session: Arc<SessionData<S>>, message: Message, args: String) -> CommandResult {
        (self.callback)(session, message, args).await
    }
}

/// The type of the callback accepted by [`crate::commands::Command`]
type CommandCallback<S> = dyn Fn(Arc<SessionData<S>>, Message, String) -> BoxFuture<'static, CommandResult> + Send + Sync;

/// The result returned by a [`Command`] when run
///
/// [`Command`]: ../commands/struct.Command.html
pub type CommandResult = Result<(), Box<dyn std::error::Error>>;

mod index;
#[doc(inline)]
pub use index::{CommandsIndex, handle_commands};

