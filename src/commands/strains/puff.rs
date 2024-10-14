use std::time::Duration;

use poise::{
    serenity_prelude::{
        self as serenity, Color, CreateActionRow, CreateButton, CreateEmbed,
        CreateInteractionResponseMessage, GuildId, ReactionType, UserId,
    },
    CreateReply,
};
use sqlx::{types::BigDecimal, PgPool};

use crate::types::{CommandError, CommandResult, Context};
use anyhow::anyhow;

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

/// Take a puff, incrementing your total puffs by one
///
/// If used in a guild, it will also increment guild puffs by one
#[poise::command(slash_command, rename = "take", member_cooldown = 30)]
pub async fn puff_take(ctx: Context<'_>) -> CommandResult {
    let user_id = ctx.author().id;

    let pool = &ctx.data().pool;

    insert_user(pool, &user_id).await;
    let user_puffs = increment_user_puffs(pool, &user_id).await?;
    let mut embed = CreateEmbed::new()
        .title(format!("Total puffs - {}", user_puffs))
        .color(Color::PURPLE);

    if let Some(guild_id) = ctx.guild_id() {
        insert_guild(pool, &guild_id).await;
        let guild_puffs = increment_guild_puffs(pool, &guild_id).await?;
        embed = embed.description(format!("Total server puffs - {}", guild_puffs));
    }

    let reply = CreateReply::default().embed(embed);

    ctx.send(reply).await?;
    Ok(())
}

async fn insert_user(pool: &PgPool, user_id: &UserId) -> bool {
    let result = sqlx::query!(
        "INSERT INTO discord.users (user_id) VALUES ($1) ON CONFLICT DO NOTHING;",
        BigDecimal::from(user_id.get())
    )
    .execute(pool)
    .await;

    result.is_ok()
}

async fn increment_user_puffs(pool: &PgPool, user_id: &UserId) -> Result<i64, CommandError> {
    sqlx::query_scalar!(
        r#"
        UPDATE discord.users
        SET puffs= puffs + 1
        WHERE
            user_id = $1
            AND NOT is_blacklisted
        RETURNING puffs;"#,
        BigDecimal::from(user_id.get())
    )
    .fetch_one(pool)
    .await
    .map_err(|_| anyhow!("Failed to update user puffs"))
}

async fn increment_guild_puffs(pool: &PgPool, guild_id: &GuildId) -> Result<i64, CommandError> {
    sqlx::query_scalar!(
        r#"
        UPDATE discord.guilds
        SET puffs = puffs + 1
        WHERE
            guild_id = $1
            AND NOT is_blacklisted
        RETURNING puffs;"#,
        BigDecimal::from(guild_id.get())
    )
    .fetch_one(pool)
    .await
    .map_err(|_| anyhow!("Failed to update guild puffs"))
}

async fn insert_guild(pool: &PgPool, guild_id: &GuildId) -> bool {
    let result = sqlx::query!(
        "INSERT INTO discord.guilds (guild_id) VALUES ($1) ON CONFLICT DO NOTHING;",
        BigDecimal::from(guild_id.get())
    )
    .execute(pool)
    .await;

    result.is_ok()
}

/// Reset your puff count to 0!
#[poise::command(slash_command, rename = "reset", user_cooldown = 30)]
pub async fn puff_reset(ctx: Context<'_>) -> CommandResult {
    let initial_embed = CreateEmbed::default().title("Are you sure?").description(
        "Resetting your puff count is an irreversable action. This will set your puff count to 0",
    );

    let cancel_id = format!("{}-puff-reset-cancel", ctx.id());
    let confirm_id = format!("{}-puff-reset-confirm", ctx.id());

    let buttons = vec![
        CreateButton::new(&cancel_id)
            .style(poise::serenity_prelude::ButtonStyle::Primary)
            .label("Cancel")
            .emoji(ReactionType::Unicode("↩️".into())),
        CreateButton::new(&confirm_id)
            .label("Reset")
            .style(poise::serenity_prelude::ButtonStyle::Danger)
            .emoji(ReactionType::Unicode("🗑".into())),
    ];

    let reply = CreateReply::default()
        .embed(initial_embed)
        .components(vec![CreateActionRow::Buttons(buttons)]);

    ctx.send(reply).await?;

    while let Some(button_interaction) = serenity::ComponentInteractionCollector::new(ctx)
        .author_id(ctx.author().id)
        .custom_ids(vec![cancel_id.clone(), confirm_id.clone()])
        .timeout(Duration::from_secs(30))
        .await
    {
        if button_interaction.data.custom_id.eq(&confirm_id) {
            let result = sqlx::query!(
                "UPDATE discord.users SET puffs = 0 WHERE user_id = $1",
                BigDecimal::from(ctx.author().id.get())
            )
            .execute(&ctx.data().pool)
            .await;

            let mut embed = CreateEmbed::default();

            if result.is_ok() {
                embed = embed.title("Puffs Reset to 0!").color(Color::PURPLE);
            } else {
                embed = embed.title("Failed to reset puffs").color(Color::RED);
            }

            button_interaction
                .create_response(
                    ctx,
                    serenity::CreateInteractionResponse::UpdateMessage(
                        CreateInteractionResponseMessage::default()
                            .embed(embed)
                            .components(vec![]),
                    ),
                )
                .await?;
            break;
        } else {
            button_interaction
                .create_response(
                    ctx,
                    serenity::CreateInteractionResponse::UpdateMessage(
                        CreateInteractionResponseMessage::default()
                            .embed(CreateEmbed::default().title("Canceled"))
                            .components(vec![]),
                    ),
                )
                .await?;
            break;
        }
    }

    Ok(())
}
