use poise::{
    serenity_prelude::{Color, CreateActionRow, CreateButton, CreateEmbed, ReactionType},
    CreateReply,
};

use crate::types::{CommandResult, Context};

#[poise::command(slash_command, required_bot_permissions = "SEND_MESSAGES")]
pub async fn about(ctx: Context<'_>) -> CommandResult {
    let embed = CreateEmbed::default()
        .title("Hi, I'm BudBuddy")
        .description("The official discord bot for BudCenter services.\n\nTry </help:1290103869791146077> for more commands")
        .color(Color::PURPLE);

    let support_button = CreateButton::new_link("https://discord.gg/GjzwzDuD3S")
        .emoji(ReactionType::Unicode("❓".into()))
        .label("Support");

    let github_button = CreateButton::new_link("https://github.com/budcenter/budbuddy")
        .emoji(ReactionType::Custom {
            animated: false,
            id: 1290481520686927935.into(),
            name: Some("blurple_github".into()),
        })
        .label("GitHub");

    let action_row = CreateActionRow::Buttons(vec![support_button, github_button]);

    let reply = CreateReply::default()
        .embed(embed)
        .ephemeral(true)
        .components(vec![action_row]);

    ctx.send(reply).await?;

    Ok(())
}
