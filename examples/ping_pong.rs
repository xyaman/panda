#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = panda::new("NjEwOTA5ODcwMDc3NjQwODUx.XswTGg.Ib3ukfR0-4eTskTaCQWL72gUZho").await?;

    client.on_message_create(|s, msg| async move {
        if msg.content == "!ping" {
            msg.send_message(&s.http, "!pong").await?;
        }

        Ok(())
    });

    client.start().await?;
    Ok(())
}
