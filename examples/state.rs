use std::{
    collections::HashMap,
    error::Error
};
use panda::{Session, events::GuildMemberAdd, HandlerResult };

// We use futures Mutex to prevent lock the thread
use futures::lock::Mutex;

#[derive(Default)]
struct State {
    guild_members: Mutex<HashMap<String, u64>>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = panda::new_with_state("your token here", State::default()).await?;
    
    client.on_guild_member_add(member_add_handler);
    client.start().await?;

    Ok(())
}

async fn member_add_handler(session: Session<State>, member: GuildMemberAdd) -> HandlerResult {
    
    // member guild_id returns Option<&str>
    let guild_id = member.guild_id().unwrap();
    
    // Get the state
    let mut counter = session.state.guild_members.lock().await;

    // Get count, set default to 0
    let guild_count = counter.entry(guild_id.to_owned()).or_insert(0);

    println!("A new member joined server {}, current count: {}", guild_id, guild_count);

    Ok(())
}