#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = panda::new("token").await?;

    client.on_message_create(|s, msg| async move {
        if msg.content() == "!ping" {
            s.send_message(msg.channel_id(), "!pong")
                .await
                .expect("Can't send message");
        }
    });
    client.start().await?;

    Ok(())
}
