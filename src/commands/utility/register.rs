use crate::types::{CommandResult, Context};

#[poise::command(prefix_command, owners_only, hide_in_help)]
pub async fn register(ctx: Context<'_>) -> CommandResult {
    poise::samples::register_application_commands_buttons(ctx).await?;
    Ok(())
}
