use panda::models::{Activity, ActivityKind, StatusUpdate};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = panda::new("your token here").await?;

    client.on_ready(|s, _| async move {
        // Create a new status struct
        let mut status = StatusUpdate::new();

        // Create a new activity
        let activity = Activity::new(ActivityKind::Listening, "!help");

        // Add activity to the new status
        status.set_activity(activity);

        // Update the status
        s.update_status(status).await?;

        Ok(())
    });

    client.start().await?;
    Ok(())
}
