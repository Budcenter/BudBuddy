use poise::CreateReply;

use crate::types::{CommandResult, Context};

/// Check bot latency
#[poise::command(slash_command, category = "Utility")]
pub async fn ping(ctx: Context<'_>) -> CommandResult {
    let ping_before = std::time::SystemTime::now();
    let ping_msg = ctx
        .say("<a:Loading:1290442390338797650> Loading...")
        .await?;

    let msg = format!("Current Latency: {}ms", ping_before.elapsed()?.as_millis());

    ping_msg
        .edit(
            ctx,
            CreateReply::default().content(msg.as_str()).ephemeral(true),
        )
        .await?;

    Ok(())
}
