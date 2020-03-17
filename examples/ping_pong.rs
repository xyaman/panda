#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = panda::new("NjEwOTA5ODcwMDc3NjQwODUx.Xm1gpA.ctIHp3Qtpd7KwRzVp5Ay6Ux8iSI").await?;

    client.on_message_create(|s, msg| async move {
        if msg.content() == "!ping" {
            msg.send_message(&s.http, "!pong").await?;
        }

        Ok(())
    });

    client.start().await?;
    Ok(())
}
