use poise::serenity_prelude::ChannelId;
use sqlx::{PgPool, postgres::PgConnectOptions};
use tracing::{debug, error, instrument, warn};

#[derive(Debug)]
pub struct Data {
    pub pool: PgPool,
    pub error_channel: Option<ChannelId>,
}

#[instrument]
async fn connect_to_db() -> PgPool {
    debug!(database_url = std::env::var("DATABASE_URL").ok());

    let pg_options = PgConnectOptions::new();

    debug!("{:#?}", pg_options);

    match PgPool::connect_with(pg_options).await {
        Ok(pool) => pool,
        Err(error) => {
            error!("sqlx::Error::{:?}", error);
            std::process::exit(1)
        }
    }
}

impl Data {
    /// Panics if it cannot connect to the database from the environment variables
    pub async fn new() -> Self {
        let mut channel: Option<ChannelId> = None;
        if let Ok(id) = std::env::var("ERROR_CHANNEL_ID") {
            if let Ok(parsed) = id.parse::<u64>() {
                channel = Some(ChannelId::new(parsed))
            }
        } else {
            warn!("ERROR_CHANNEL_ID not set");
        }
        Self {
            pool: connect_to_db().await,
            error_channel: channel,
        }
    }
}

pub type CommandError = anyhow::Error;
pub type Command = poise::Command<Data, CommandError>;
pub type CommandResult = Result<(), CommandError>;
pub type Context<'a> = poise::Context<'a, Data, CommandError>;
