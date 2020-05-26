// This example shows how to send an embed, how actually handler functions are.
// * HandlerResult * it's just an alias for Result<(), Box<dyn std::error::Error>>

use panda::{events::MessageCreate, models::Embed, HandlerResult, Session};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = panda::new("your token here").await?;

    client.on_message_create(message_handler);
    client.start().await?;

    Ok(())
}

async fn message_handler(s: Session<()>, msg: MessageCreate) -> HandlerResult {
    if msg.content == "!embed" {
        let mut embed = Embed::new();
        embed
            .set_title("Some title")
            .set_description("Some description here")
            .add_field("Regular field title", "Some value here", false)
            .add_field("Inline field title", "inline content", true)
            .add_field("Inline field title", "inline content", true);

        if let Err(e) = msg.send_embed(&s.http, embed).await {
            println!("Error when sending embed: {}", e);
        }
    }

    Ok(())
}
