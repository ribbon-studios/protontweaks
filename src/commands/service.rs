use clap::Args;
use cli_prompts_rs::{CliPrompt, PromptSelectOption};
use owo_colors::OwoColorize;

use crate::utils::service::{register, unregister};

#[derive(Debug, Args)]
pub struct CommandArgs {
    /// Uninstalls the watch service
    #[arg(long)]
    pub uninstall: bool,

    /// Installs the watch service
    #[arg(long)]
    pub install: bool,
}

pub async fn command(args: CommandArgs) -> Result<(), String> {
    println!("{}", "Protontweaks service management".bold());

    let mut cli_prompt = CliPrompt::new();

    let install = if args.install {
        true
    } else if args.uninstall {
        false
    } else {
        let selected_option = cli_prompt
            .prompt_select(
                "Would you like to install or uninstall the watch service?",
                vec![
                    PromptSelectOption::new("install", "Install"),
                    PromptSelectOption::new("uninstall", "Uninstall"),
                ],
            )
            .map_err(|e| e.to_string())?;

        selected_option.value == "install"
    };

    if install {
        println!("{}", "Registering service...".bold());
        register().await?;
        println!("{}", "Service registered successfully!".green());
    } else {
        println!("{}", "Removing service...".bold());
        unregister().await?;
        println!("{}", "Service removed successfully!".green());
    }

    Ok(())
}
