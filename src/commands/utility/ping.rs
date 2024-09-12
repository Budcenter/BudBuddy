use poise::{
    serenity_prelude::{self as serenity, Color, CreateEmbed, Embed},
    CreateReply,
};

use crate::types::{Command, CommandError, Context};

/// Responds with an embed to verify online status
#[poise::command(
    slash_command,
    required_bot_permissions = "SEND_MESSAGES",
    category = "Utility"
)]
pub async fn ping(ctx: Context<'_>) -> Result<(), anyhow::Error> {
    let mut reply = CreateReply::default();
    let mut embed = CreateEmbed::default().title("Pong!");
    reply = reply.embed(embed);
    ctx.send(reply).await?;
    Ok(())
}
