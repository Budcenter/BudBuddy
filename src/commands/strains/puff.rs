use crate::types::{CommandResult, Context};

/// Track puffs on the leaderboard
#[poise::command(
    slash_command,
    category = "Strains",
    subcommands("puff_take", "puff_reset"),
    subcommand_required
)]
pub async fn puff(_ctx: Context<'_>) -> CommandResult {
    // This command cannot be called by the client because of Discord's subcommand system.
    // See https://discord.com/developers/docs/interactions/application-commands#subcommands-and-subcommand-groups
    Ok(())
}

#[poise::command(slash_command)]
pub async fn puff_take(_ctx: Context<'_>) -> CommandResult {
    todo!();
}

#[poise::command(slash_command)]
pub async fn puff_reset(_ctx: Context<'_>) -> CommandResult {
    todo!();
}
