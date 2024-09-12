#![allow(unused)]

use anyhow::anyhow;
use std::default;

use poise::{serenity_prelude as serenity, FrameworkOptions};
use tracing::{debug, error, info, instrument};
use types::Data;

pub mod commands;
pub mod types;

// Sets the log level for the bot. Default: INFO
const LEVEL: tracing::Level = tracing::Level::INFO;

#[tokio::main]
#[instrument]
async fn main() -> Result<(), anyhow::Error> {
    let token = dotenvy::var("DISCORD_TOKEN")?;

    tracing_subscriber::fmt().with_max_level(LEVEL).init();

    info!("Starting Bot...");

    let intents = serenity::GatewayIntents::non_privileged();

    let commands = vec![
        commands::utility::ping::ping(),
        commands::utility::help::help(),
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
                info!("Online");
                Ok(Data {})
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
