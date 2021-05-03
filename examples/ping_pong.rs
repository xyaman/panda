#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = panda::new("your token here").await?;

    // Run this function every time a message is received
    client.on_message_create(|s, msg| async move {
        // Only respond if the message is !ping
        if msg.content == "!ping" {
            msg.send(&s.http, "!pong").await?;
        }

        Ok(())
    });

    client.start().await?;
    Ok(())
}
