use anyhow::anyhow;
use std::{env::VarError, process::exit};
use tracing_subscriber::EnvFilter;

use poise::{
    serenity_prelude::{self as serenity},
    FrameworkOptions,
};
use tracing::{error, info, instrument};
use types::Data;

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
        Err(e) => {
            error!(?e);
            exit(1);
        }
    };

    info!("Starting Bot...");

    let intents = serenity::GatewayIntents::non_privileged();

    let commands = vec![
        commands::utility::ping::ping(),
        commands::utility::help::help(),
        commands::strains::search_strains::search(),
        commands::utility::register::register(),
    ];

    let framework_options = FrameworkOptions {
        commands,
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
