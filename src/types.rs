use sqlx::{
    postgres::{PgConnectOptions, PgSslMode},
    PgPool,
};
use tracing::{debug, error, instrument};

use crate::unwrap_env_var;

pub struct Data {
    pub pool: PgPool,
}
#[instrument]
async fn connect_to_db() -> PgPool {
    debug!(database_url = dotenvy::var("DATABASE_URL").ok());

    let pg_options = PgConnectOptions::new()
        .ssl_mode(PgSslMode::VerifyFull)
        .ssl_root_cert(unwrap_env_var("SSL_CERTIFICATE"));

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
        Self {
            pool: connect_to_db().await,
        }
    }
}

pub type CommandError = anyhow::Error;
pub type Command = poise::Command<Data, CommandError>;
pub type CommandResult = Result<(), CommandError>;
pub type Context<'a> = poise::Context<'a, Data, CommandError>;
