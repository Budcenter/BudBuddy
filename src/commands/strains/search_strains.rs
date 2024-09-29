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
pub async fn search(ctx: Context<'_>, name: Option<String>, subspecies: Option<Subspecies>) -> CommandResult {

    let pool = &ctx.data().pool;

    let result = sqlx::query!(
        r#"
        SELECT
            id,
            NAME,
            description,
            image_url,
            subspecies AS "subspecies:String"
        FROM
            strains
        WHERE
            (NAME ILIKE COALESCE('%' || $1 || '%', NAME))
            AND (subspecies = COALESCE($2, subspecies))
        LIMIT
            15;
        "#,
        name,
        subspecies as _,
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
