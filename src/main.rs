use anyhow::anyhow;
use std::{env::VarError, process::exit, str::FromStr};
use tracing_subscriber::EnvFilter;

use poise::{
    serenity_prelude::{self as serenity, Color, CreateEmbed, CreateEmbedAuthor, CreateMessage},
    CreateReply, FrameworkOptions,
};
use tracing::{error, info, instrument, warn};
use types::{CommandError, Context, Data};

pub mod commands;
pub mod types;

#[tokio::main]
#[instrument]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let bot_data = Data::new().await;

    let token = unwrap_env_var("DISCORD_TOKEN");

    let _guild_id = match unwrap_env_var("GUILD_ID").parse() {
        Ok(v) => serenity::GuildId::new(v),
        Err(error) => {
            error!(?error);
            exit(1);
        }
    };

    info!("Starting Bot...");

    let intents = serenity::GatewayIntents::non_privileged();

    let commands = vec![
        commands::utility::ping::ping(),
        commands::utility::help::help(),
        commands::strains::search_strains::search(),
        commands::strains::fetch_strain::strain(),
        commands::utility::register::register(),
        commands::utility::about::about(),
    ];

    let framework_options = FrameworkOptions {
        commands,
        command_check: Some(|ctx| Box::pin(global_command_check(ctx))),
        on_error: |error| Box::pin(global_error_handler(error)),
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .options(framework_options)
        .setup(move |ctx, ready, framework| {
            Box::pin(async move {
                info!("Registering commands...");
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                info!("Online on bot: {} ({})", ready.user.name, ready.user.id);
                Ok(bot_data)
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .map_err(|e| anyhow!(e))?;

    client.start().await.expect("Failed to start bot");
    Ok(())
}

fn unwrap_env_var(name: &str) -> String {
    match std::env::var(name) {
        Ok(t) => t,
        Err(VarError::NotPresent) => {
            error!("{name} not set");
            exit(1)
        }
        Err(VarError::NotUnicode(s)) => {
            error!("{name} is not valid unicode: {:#?}", s);
            exit(1)
        }
    }
}

fn error_embed(title: &str, message: Option<&str>) -> CreateEmbed {
    let mut embed = CreateEmbed::default().color(Color::RED).title(title);

    if message.is_some() {
        embed = embed.description(message.unwrap());
    }
    embed
}

fn error_reply(title: &str, message: Option<&str>) -> CreateReply {
    let embed = error_embed(title, message);

    CreateReply::default().embed(embed).ephemeral(true)
}

async fn global_command_check(ctx: Context<'_>) -> Result<bool, CommandError> {
    let id = sqlx::types::BigDecimal::from_str(&ctx.author().id.to_string())?;
    let is_user_blacklisted = sqlx::query_scalar!(
        r#"
        SELECT
            is_blacklisted
        FROM
            discord.users
        WHERE
            user_id = $1;"#,
        id
    )
    .fetch_optional(&ctx.data().pool)
    .await
    .unwrap_or_default();

    Ok(!is_user_blacklisted.unwrap_or(false))
}

async fn global_error_handler(error: poise::FrameworkError<'_, Data, CommandError>) {
    if let poise::FrameworkError::CommandCheckFailed { ctx, .. } = error {
        ctx.send(error_reply("Blacklisted", None)).await.ok();
    } else if let poise::FrameworkError::CooldownHit {
        remaining_cooldown,
        ctx,
        ..
    } = error
    {
        ctx.send(error_reply(
            &format!("/{} on cooldown", ctx.command().qualified_name),
            Some(&format!(
                "Please wait {:.1} seconds before trying again",
                remaining_cooldown.as_secs_f32()
            )),
        ))
        .await
        .ok();
    } else if let poise::FrameworkError::NsfwOnly { ctx, .. } = error {
        ctx.send(error_reply(
            "NSFW Only Command",
            Some(&format!(
                "`/{}` can only be used in NSFW channels",
                ctx.command().identifying_name,
            )),
        ))
        .await
        .ok();
    } else if let Some(ctx) = error.ctx() {
        if let Some(channel) = ctx.data().error_channel {
            let mut embed = CreateEmbed::new()
                .title(format!(
                    "Unknown error occured in /{}",
                    ctx.command().qualified_name
                ))
                .description(format!("{}", error))
                .color(Color::RED);
            if let Some(guild) = ctx.guild_channel().await {
                embed = embed.fields([
                    (
                        "Guild",
                        format!(
                            "**{}** - `{}`",
                            guild
                                .guild_id
                                .name(ctx.cache())
                                .unwrap_or("[Unknown]".into()),
                            guild.guild_id
                        ),
                        true,
                    ),
                    (
                        "Channel",
                        format!("**{}** - `{}`", guild.name, guild.id),
                        true,
                    ),
                ]);
            }
            let user = ctx.author();
            embed = embed.author(CreateEmbedAuthor::new(&format!(
                "{} - `{}`",
                user.name, user.id
            )));
            channel
                .send_message(ctx.http(), CreateMessage::new().embed(embed))
                .await
                .ok();

            warn!("Unknown error occured: {:#?}", error);
        } else {
            warn!("Error Channel Missing - Error: {:#?}", error)
        }
    } else {
        warn!("Unkown Error Occured: {:#?}", error)
    }
}
