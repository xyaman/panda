use panda::client::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = panda::new("your token here").await?;

    // Create config struct
    let config = Config::new().set_large_threshold(100);

    // Before start bot, set a confi
    client.set_config(config)?;

    client.start().await?;
    Ok(())
}
