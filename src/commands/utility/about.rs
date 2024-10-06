use poise::{
    serenity_prelude::{Color, CreateActionRow, CreateButton, CreateEmbed, ReactionType},
    CreateReply,
};
use sqlx::PgPool;

use crate::types::{CommandResult, Context};

struct StrainTotals {
    total: i64,
    hybrid: i64,
    indica: i64,
    sativa: i64,
}

impl StrainTotals {
    fn unknown(&self) -> i64 {
        self.total - self.hybrid - self.indica - self.sativa
    }
}

static STRAIN_COUNTS: tokio::sync::OnceCell<StrainTotals> = tokio::sync::OnceCell::const_new();

async fn strain_counts(pool: &PgPool) -> StrainTotals {
    let totals = sqlx::query!(
        r#"
        SELECT
        COUNT(*) AS "total_strains!",
        COUNT(
        CASE
        WHEN subspecies = 'hybrid' THEN 1
        END
        ) AS "total_hybrid!",
        COUNT(
        CASE
        WHEN subspecies = 'sativa' THEN 1
        END
        ) AS "total_sativa!",
        COUNT(
        CASE
        WHEN subspecies = 'indica' THEN 1
        END
        ) AS "total_indica!"
        FROM
        public.strains;
    "#
    )
    .fetch_one(pool)
    .await
    .unwrap();

    StrainTotals {
        total: totals.total_strains,
        hybrid: totals.total_hybrid,
        indica: totals.total_indica,
        sativa: totals.total_sativa,
    }
}

#[poise::command(slash_command, required_bot_permissions = "SEND_MESSAGES")]
pub async fn about(ctx: Context<'_>) -> CommandResult {
    let counts = STRAIN_COUNTS
        .get_or_init(|| async { strain_counts(&ctx.data().pool).await })
        .await;
    let embed = CreateEmbed::default()
        .title("Hi, I'm BudBuddy")
        .description("The official discord bot for BudCenter services.\n\nTry </help:1290103869791146077> for more commands")
        .color(Color::PURPLE)
        .fields([
            ("Credits", format!("- {} - Lead Developer\n- {} - Cannabot Developer", "@makeshiftartist", "@jay.0404"), true),
            ("Strains", format!("- `{}` Total\n- `{}` Indica\n- `{}` Sativa\n- `{}` Hybrid\n- `{}` Unknown", counts.total, counts.indica, counts.sativa, counts.hybrid, counts.unknown()), false),
        ]);

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
