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
                    AND e.is_positive IS TRUE
            ) AS positive_effects,
            ARRAY (
                SELECT e.effect
                FROM public.strain_effects se
                JOIN public.unique_effects e ON se.effect_id = e.id
                WHERE
                    se.strain_id = s.id
                    AND e.is_positive IS FALSE
            ) AS negative_effects,
            ARRAY (
                SELECT
                    f.flavor
                FROM public.strain_flavors sf
                JOIN public.unique_flavors f ON sf.flavor_id = f.id
                WHERE
                    sf.strain_id = s.id
            ) AS flavors,
            ARRAY (
                SELECT
                    a.ailment
                FROM public.strain_ailments sa
                JOIN public.unique_ailments a ON sa.ailment_id = a.id
                WHERE
                    sa.strain_id = s.id
            ) AS ailments
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

    if let Some(s) = strain.subspecies {
        embed = embed.field("🎨 Subspecies", s.to_string(), false);
    }

    if let Some(s) = strain.positive_effects.as_ref() {
        if !s.is_empty() {
            let effects = s.join(", ");

            embed = embed.field("🔺 Positive Effects", effects, false);
        }
    }

    if let Some(s) = strain.negative_effects.as_ref() {
        if !s.is_empty() {
            let effects = s.join(", ");

            embed = embed.field("🔻 Negative Effects", effects, true);
        }
    }

    if let Some(s) = strain.flavors.as_ref() {
        if !s.is_empty() {
            let effects = s.join(", ");

            embed = embed.field("👅 Flavors", effects, false);
        }
    }

    if let Some(s) = strain.ailments.as_ref() {
        if !s.is_empty() {
            let effects = s.join(", ");

            embed = embed.field("💊 Ailments", effects, false);
        }
    }

    if let Some(image) = strain.image_url {
        embed = embed.image(image);
    }

    let reply = CreateReply::default().embed(embed);
    ctx.send(reply).await?;
    Ok(())
}
