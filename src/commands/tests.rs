use std::sync::Arc;

use crate::client::SessionData;
use crate::commands::{Command, CommandsIndex, CommandResult};
use crate::models::channel::Message;


async fn hello(_session: Arc<SessionData<()>>, _msg: Message, _args: String) -> CommandResult {
    println!("Hello");
    Ok(())
}

fn empty_index() -> CommandsIndex<()> {
    CommandsIndex::<()>::new("?")
}

fn filled_index() -> CommandsIndex<()> {
    let mut index = empty_index();
    let hello = Command::new(hello);
    index.command("hello", hello).unwrap();
    index
}

#[test]
fn commands_index_command_returns_err() {
    let mut index = empty_index();

    // Error : "hello world" is not a valid name for a command because whitespaces are forbidden
    let hello = Command::new(hello);
    assert!(index.command("hello world", hello).is_err());
}

#[test]
fn commands_index_command_returns_ok() {
    let mut index = empty_index();

    // Ok : "hello" is a valid name for a command
    let hello = Command::new(hello);
    assert!(index.command("hello", hello).is_ok());
}

#[test]
fn commands_index_parse_bad_prefix() {
    let index = filled_index();

    // Wrong prefix : the user shall write `?hello` instead
    assert!(index.parse("!hello").is_none());
}

#[test]
fn commands_index_parse_bad_command() {
    let index = filled_index();

    // Wrong command
    assert!(index.parse("?hell").is_none());
}

#[test]
fn commands_index_parse_no_args() {
    let index = filled_index();
    let (_command, args) = index.parse("?hello").unwrap();

    assert!(args.is_empty());
}

#[test]
fn commands_index_parse_no_args_with_trailing_whitespace() {
    let index = filled_index();
    let (_command, args) = index.parse("?hello ").unwrap();

    assert!(args.is_empty());
}

#[test]
fn commands_index_parse_with_args() {
    let index = filled_index();
    let (_command, args) = index.parse("?hello world").unwrap();

    assert_eq!(args, "world");
}

