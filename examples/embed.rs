use panda::models::channel::Embed;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut client = panda::new("NjEwOTA5ODcwMDc3NjQwODUx.Xm1gpA.ctIHp3Qtpd7KwRzVp5Ay6Ux8iSI").await?;

    client.on_message_create(|s, m| async move {
        
        if m.content() == "!embed" {
            let mut embed = Embed::new();
            embed.set_title("New Embed")
                 .set_description("Embed description");

            if let Err(e) = s.http.send_embed(m.channel_id(), embed).await {
                println!("Error {}", e);
            }
        }

        Ok(())
    });

    client.start().await?;

    Ok(())
}