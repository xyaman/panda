use panda::{events::MessageCreate, models::Embed, Session};
use std::{error::Error, sync::Arc};

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = panda::new("token here").await?;

    client.on_message_create(message_handler);
    client.start().await?;

    Ok(())
}

async fn message_handler(s: Arc<Session>, message: MessageCreate) -> Result<(), Box<dyn Error>> {
    if message.content() == "!embed" {
        let mut embed = Embed::new();
        embed
            .set_title("Some title")
            .set_description("Some description here")
            .add_field("Regular field title", "Some value here", false)
            .add_field("Inline field title", "inline content", true);

        if let Err(e) = message.send_embed(&s.http, embed).await {
            println!("Error {}", e);
        }
    }
    Ok(())
}
