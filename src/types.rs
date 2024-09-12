pub struct Data {}

impl Data {
    pub fn new() -> Self {
        Self {}
    }
}

pub type CommandError = anyhow::Error;
pub type Command = poise::Command<Data, CommandError>;
pub type Context<'a> = poise::Context<'a, Data, CommandError>;
