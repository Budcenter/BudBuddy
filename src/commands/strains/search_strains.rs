use poise::{
    serenity_prelude::CreateEmbed, ChoiceParameter, CreateReply
};
use serde::{Deserialize, Serialize};
use sqlx::Type;

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
    category = "Strains"
)]
pub async fn search(ctx: Context<'_>, name: Option<String>, subspecies: Option<Subspecies>, flavor: Option<String>) -> CommandResult {

    let pool = &ctx.data().pool;

    let result = sqlx::query!(
        r#"
        SELECT
            s.id,
            s.NAME,
            s.description,
            s.image_url,
            s.subspecies AS "subspecies:String"
        FROM
            public.strains s
            LEFT JOIN public.strain_flavors sf ON s.id = sf.strain_id
            LEFT JOIN public.unique_flavors uf ON sf.flavor_id = uf.id
        WHERE
            (s.NAME ILIKE COALESCE('%' || $1 || '%', s.NAME))
            AND (s.subspecies = COALESCE($2, s.subspecies))
            AND (uf.flavor ILIKE (COALESCE($3, uf.flavor)) OR $3 IS NULL)
        LIMIT
            15;
        "#,
        name,
        subspecies as _,
        flavor
    )
    .fetch_all(pool)
    .await?;

    let mut description = String::new();

    for strain in result {
        description.push_str(&format!("- `{}`: **{}**", strain.id, strain.name));
        description.push_str("\n");
    }

    let title = match name {
        Some(t) => format!("Searching for {t}"),
        None => "Strains".to_string()
    };


    let embed = CreateEmbed::default().title(title).description(description);
    let reply = CreateReply::default().embed(embed);
    ctx.send(reply).await?;
    Ok(())

}
