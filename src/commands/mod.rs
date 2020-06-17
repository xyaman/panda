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
//! ```
//! use std::sync::Arc;
//!
//! use panda::client::SessionData;
//! use panda::commands::{Command, CommandResult, CommandsIndex};
//! use panda::models::channel::Message;
//!
//! async fn pong(session: Arc<SessionData<()>>, msg: Message) -> CommandResult {
//!     msg.send(&session.http, "Pong").await?;
//!     Ok(())
//! }
//!
//! async fn handler(index: Arc<CommandsIndex<()>>, session: Arc<SessionData<()>>, msg: Message) ->
//! Result<(), Box<dyn std::error::Error>> {
//!     let cmd = match index.parse(&msg.content) {
//!         Some(val) => val,
//!         None      => return Ok(()),
//!     };
//!
//!     cmd.run(session, msg).await
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut index = CommandsIndex::new("?");
//!     index.command("ping", Command::new(pong));
//!     let index = Arc::new(index);
//!
//!     let mut bot = panda::new("your token here").await.unwrap();
//!     bot.on_message_create(move |session, event| {
//!         handler(index.clone(), session, event.0)
//!     });
//!
//!     bot.start().await;
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
        F: Fn(Arc<SessionData<S>>, Message) -> Fut,
        Fut: Send + 'static,
        Fut: Future<Output = CommandResult>,
    {
        let pinned = move |session, msg| -> BoxFuture<'static, CommandResult> {
            Box::pin(callback(session, msg))
        };

        Self {
            callback: Box::new(pinned),
        }
    }

    /// Runs `self`
    pub async fn run(&self, session: Arc<SessionData<S>>, message: Message) -> CommandResult {
        (self.callback)(session, message).await
    }
}

/// The type of the callback accepted by [`crate::commands::Command`]
type CommandCallback<S> = dyn Fn(Arc<SessionData<S>>, Message) -> BoxFuture<'static, CommandResult> + Send + Sync;

/// The result returned by a [`Command`] when run
///
/// [`Command`]: ../commands/struct.Command.html
pub type CommandResult = Result<(), Box<dyn std::error::Error>>;

mod index;
#[doc(inline)]
pub use index::CommandsIndex;
