use poise::{
    serenity_prelude::{Color, CreateEmbed},
    ChoiceParameter, CreateReply,
};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use tokio::sync::OnceCell;
use tracing::info;

use crate::types::{CommandResult, Context};

#[derive(Debug, Type, Deserialize, Serialize, ChoiceParameter)]
#[sqlx(type_name = "subspecies", rename_all = "lowercase")]
pub enum Subspecies {
    Hybrid,
    Indica,
    Sativa,
    Ruderalis,
}

/// Searches strains with a filter
#[poise::command(
    slash_command,
    required_bot_permissions = "SEND_MESSAGES",
    nsfw_only = true,
    category = "Strains"
)]
pub async fn search(
    ctx: Context<'_>,
    #[description = "Name of the strain"] name: Option<String>,
    #[description = "Indica, Sativa, Hybrid, or Ruderalis"] subspecies: Option<Subspecies>,
    #[description = "Reported strain flavors"]
    #[autocomplete = "autocomplete_flavors"]
    flavor: Option<String>,
    #[description = "Reported strain effects"]
    #[autocomplete = "autocomplete_effects"]
    effect: Option<String>,
    #[description = "Reported strain ailments"]
    #[autocomplete = "autocomplete_ailments"]
    ailment: Option<String>,
) -> CommandResult {
    let pool = &ctx.data().pool;

    let result = sqlx::query!(
        r#"
        SELECT DISTINCT
            s.id,
            s.NAME
        FROM
            public.strains s

            -- Flavors
            LEFT JOIN public.strain_flavors sf ON s.id = sf.strain_id
            LEFT JOIN public.unique_flavors uf ON sf.flavor_id = uf.id

            -- Effects
            LEFT JOIN public.strain_effects se ON s.id = se.strain_id
            LEFT JOIN public.unique_effects ue ON se.effect_id = ue.id

            -- Ailments
            LEFT JOIN public.strain_ailments sa ON s.id = sa.strain_id
            LEFT JOIN public.unique_ailments ua ON sa.ailment_id = ua.id
        WHERE
            (s.NAME ILIKE COALESCE('%' || $1 || '%', s.NAME) OR $1 IS NULL)
            AND (s.subspecies = COALESCE($2, s.subspecies) OR $2 IS NULL)
            AND (uf.flavor ILIKE (COALESCE($3, uf.flavor)) OR $3 IS NULL)
            AND (ue.effect ILIKE (COALESCE($4, ue.effect)) OR $4 IS NULL)
            AND (ua.ailment ILIKE (COALESCE($5, ua.ailment)) OR $5 IS NULL)
        ORDER BY
            s.id ASC
        LIMIT
            15;
        "#,
        name,
        subspecies as _,
        flavor,
        effect,
        ailment
    )
    .fetch_all(pool)
    .await?;

    let mut description = String::new();

    if result.is_empty() {
        let embed = CreateEmbed::default()
            .title("No Strains found")
            .description("Try broadening your seach filters")
            .color(Color::RED);
        let reply = CreateReply::default().embed(embed);
        ctx.send(reply).await?;
        return Ok(());
    }

    for strain in result {
        description.push_str(&format!("- `{}`: **{}**", strain.id, strain.name));
        description.push('\n');
    }

    let title = match name {
        Some(t) => format!("Strains matching: \"{t}\""),
        None => "Strains".to_string(),
    };

    let embed = CreateEmbed::default()
        .title(title)
        .description(description)
        .color(Color::PURPLE);
    let reply = CreateReply::default().embed(embed);
    ctx.send(reply).await?;
    Ok(())
}

static FLAVORS: OnceCell<Vec<String>> = OnceCell::const_new();

async fn autocomplete_flavors(ctx: Context<'_>, searching: &str) -> Vec<String> {
    let flavors = FLAVORS
        .get_or_init(|| async {
            info!("Fetched flavors");
            sqlx::query_scalar!(
                "SELECT DISTINCT flavor FROM public.unique_flavors ORDER BY flavor ASC;"
            )
            .fetch_all(&ctx.data().pool)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|flavor| flavor.to_lowercase())
            .collect()
        })
        .await;

    flavors
        .clone()
        .into_iter()
        .filter(|flavor| flavor.starts_with(&searching.to_lowercase()))
        .collect()
}

async fn autocomplete_effects(ctx: Context<'_>, searching: &str) -> Vec<String> {
    let effects = sqlx::query_scalar!("SELECT effect FROM public.unique_effects")
        .fetch_all(&ctx.data().pool)
        .await
        .unwrap_or_default();
    effects
        .into_iter()
        .filter(|effect| effect.to_lowercase().starts_with(&searching.to_lowercase()))
        .collect()
}

async fn autocomplete_ailments(ctx: Context<'_>, searching: &str) -> Vec<String> {
    let ailments = sqlx::query_scalar!("SELECT ailment FROM public.unique_ailments")
        .fetch_all(&ctx.data().pool)
        .await
        .unwrap_or_default();
    ailments
        .into_iter()
        .filter(|ailment| {
            ailment
                .to_lowercase()
                .starts_with(&searching.to_lowercase())
        })
        .collect()
}
