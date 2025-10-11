use super::search_strains::Subspecies;
use crate::{
    error_reply,
    types::{CommandResult, Context},
};
use poise::{
    CreateReply,
    serenity_prelude::{Color, CreateEmbed, CreateEmbedFooter},
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
    #[description = "ID of the strain"] id: i32,
) -> CommandResult {
    let pool = &ctx.data().pool;

    let result = sqlx::query!(
        r#"
        SELECT
            strain.name,
            strain.description,
            strain.subspecies as "subspecies:Subspecies",
            strain.image_url,
            ARRAY (
                SELECT effect.name
                FROM cannabis.strain_effects se
                JOIN cannabis.effects effect ON se.effect_id = effect.id
                WHERE se.strain_id = strain.id
                AND effect.is_positive IS TRUE
            ) AS positive_effects,
            ARRAY (
                SELECT effect.name
                FROM cannabis.strain_effects se
                JOIN cannabis.effects effect ON se.effect_id = effect.id
                WHERE se.strain_id = strain.id
                AND effect.is_positive IS FALSE
            ) AS negative_effects,
            ARRAY (
                SELECT flavor.name
                FROM cannabis.strain_flavors sf
                JOIN cannabis.flavors flavor ON sf.flavor_id = flavor.id
                WHERE sf.strain_id = strain.id
            ) AS flavors,
            ARRAY (
                SELECT ailment.name
                FROM cannabis.strain_ailments sa
                JOIN cannabis.ailments ailment ON sa.ailment_id = ailment.id
                WHERE sa.strain_id = strain.id
            ) AS ailments
        FROM cannabis.strains strain
        WHERE strain.id = $1
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
