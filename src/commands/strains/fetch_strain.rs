use super::search_strains::Subspecies;
use crate::{
    error_reply,
    types::{CommandResult, Context},
};
use poise::{
    serenity_prelude::{Color, CreateEmbed, CreateEmbedFooter},
    CreateReply,
};
use tracing::warn;

/// Fetches a strain by it's ID
#[poise::command(
    slash_command,
    required_bot_permissions = "SEND_MESSAGES",
    nsfw_only = true,
    category = "Strains"
)]
pub async fn strain(
    ctx: Context<'_>,
    #[description = "ID of the strain"] id: i64,
) -> CommandResult {
    let pool = &ctx.data().pool;

    let result = sqlx::query!(
        r#"
        SELECT
            s.name,
            s.description,
            s.subspecies as "subspecies:Subspecies",
            s.image_url,
            ARRAY (
                SELECT
                    e.effect
                FROM public.strain_effects se
                JOIN public.unique_effects e ON se.effect_id = e.id
                WHERE
                    se.strain_id = s.id
            ) AS effects
        FROM public.strains s
        WHERE s.id = $1
        LIMIT 1;
        "#,
        id
    )
    .fetch_one(pool)
    .await;

    let strain = match result {
        Ok(s) => s,
        Err(e) => match e {
            sqlx::Error::RowNotFound => {
                ctx.send(error_reply(
                    "Strain Not Found",
                    Some(&format!("Couldn't find strain with id: `{}`", id)),
                ))
                .await?;
                return Ok(());
            }
            other => {
                warn!("Error: {:#?}", other);
                return Err(other)?;
            }
        },
    };

    let mut embed = CreateEmbed::default()
        .title(strain.name)
        .description(
            strain
                .description
                .unwrap_or("No description available".into()),
        )
        .color(Color::PURPLE)
        .footer(CreateEmbedFooter::new(format!("ID: {}", id)));

    if let Some(s) = strain.effects.as_ref() {
        let effects = s.join(", ");

        embed = embed.field("Effects", effects, true);
    }

    if let Some(image) = strain.image_url {
        embed = embed.image(image);
    }
    let reply = CreateReply::default().embed(embed);
    ctx.send(reply).await?;
    Ok(())
}
