use panda::client::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = panda::new("your token here").await?;

    let config = Config::new().set_large_threshold(100);

    client.set_config(config)?;

    client.start().await?;
    Ok(())
}
