<div align="center">
    <img width="500" src="https://i.postimg.cc/RhwNMHtZ/banner.png" />
</div>
<hr />
<div align="center">
    <!-- Crates version -->
    <a href="https://crates.io/crates/panda">
        <img src="https://img.shields.io/crates/v/panda?style=flat-square">
    </a>
    <!-- docs.rs -->
    <a href="https://docs.rs/panda">
        <img src="https://img.shields.io/badge/docs-online-blue?style=flat-square" />
    </a>
</div>
<br />

A powerful async Rust library for interacting with Discord's API

Even thought this library is usable, it still under development, so don't use for production yet.

> Note that this library doesn't support the 100% of discord API yet, for example voice. See `TODO list` to more information.

# Installation
```
cargo add panda
```

or in `Cargo.toml`

```
panda = "0.5.2"
```

# Configuring async runtime
panda supports `tokio` and `async-std` runtimes, by default it uses `tokio`,
to use `async-std` change the feature flags in `Cargo.toml`

```toml
[dependencies.panda]
version = "0.5.2"
default-features = false
features = ["async-std-runtime"]
```


# Example usage
It will print the bot name when the bot is ready.

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut client = panda::new("your token here").await?;

    client.on_ready(|s, ready| async move {
        println!("Bot {} is ready", ready.user.username);

        Ok(())
    });

    client.start().await?;

    Ok(())
}
```

All events are in the [Discord Documentation](https://discord.com/developers/docs/topics/gateway#commands-and-events), and to use it in client, you have to use `client.on_` plus 
the event in snake case.

# TODO list

- Finish http requests.
- Improve panda error.
- Add voice support.
- Improve documentation.
- Add tests
