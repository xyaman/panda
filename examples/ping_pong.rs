#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = panda::new("your token here").await?;

    client.on_message_create(|s, msg| async move {
        if msg.content() == "!ping" {
            msg.send_message(&s.http, "!pong").await?;
        }

        Ok(())
    });

    client.start().await?;
    Ok(())
}
