#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client =
        panda::new("NjEwOTA5ODcwMDc3NjQwODUx.XkMRew.mzkeFk8k2hfHMQOlxbI8Q2X0Zkw").await?;

    client.on_ready(|_, r| async move { println!("{}", r.user().username()) });
    client.on_presence_update(|_, _| async move {
        println!("Presence updated");
    });
    client.on_message_create(|session, msg| async move {
        if msg.content() == "!ping" {
            session
                .send_message(msg.channel_id(), "pong")
                .await
                .unwrap();
        }
        //%F0%9F%91%8C
        session
            .add_reaction(msg.channel_id(), msg.id(), "👍")
            .await
            .expect("can't react");
    });
    client.start().await?;

    Ok(())
}
