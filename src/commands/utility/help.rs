use poise::samples::HelpConfiguration;

use crate::types::{CommandResult, Context};

/// Show help message
#[poise::command(prefix_command, slash_command, category = "Utility")]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Command to get help for"]
    #[autocomplete = "autocomplete_commands"]
    mut command: Option<String>,
) -> CommandResult {
    // This makes it possible to just make `help` a subcommand of any command
    // `/fruit help` turns into `/help fruit`
    // `/fruit help apple` turns into `/help fruit apple`
    if ctx.invoked_command_name() != "help" {
        command = match command {
            Some(c) => Some(format!("{} {}", ctx.invoked_command_name(), c)),
            None => Some(ctx.invoked_command_name().to_string()),
        };
    }

    let config = HelpConfiguration {
        show_subcommands: true,
        show_context_menu_commands: false,
        ephemeral: true,
        include_description: true,
        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), config).await?;
    Ok(())
}

async fn autocomplete_commands(ctx: Context<'_>, searching: &str) -> Vec<String> {
    let mut result = Vec::new();
    for command in &ctx.framework().options().commands {
        if command.owners_only || command.hide_in_help {
            continue;
        };

        if command.qualified_name.starts_with(searching) {
            result.push(command.qualified_name.clone())
        }
    }

    result
}
