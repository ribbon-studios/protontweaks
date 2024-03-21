use cli_prompts_rs::CliPrompt;
use owo_colors::OwoColorize;

use crate::{
    config::{detect_valid_home, try_save, Config},
    utils::service::register,
};

pub async fn command() -> Result<(), String> {
    println!("{}", "~ Protontweaks Setup ~".blue().bold());

    let mut cli_prompt = CliPrompt::new();

    let service = if os_info::get().to_string().contains("NixOS") {
        println!(
            "{} {}",
            "[service] ->",
            "skipping due to read only file system...".italic()
        );
        false
    } else {
        cli_prompt
            .prompt_confirm("Would you like to install the watch service?")
            .map_err(|e| e.to_string())?
    };

    if service {
        println!("{}", "Registering service...".bold());
        register().await?;
        println!("{}", "Service registered successfully!".green());
    }

    let config = cli_prompt
        .prompt_confirm("Would you like us to initialize a config for you?")
        .map_err(|e| e.to_string())?;

    if config {
        let gamemode = cli_prompt
            .prompt_confirm(
                "Would you like us to automatically run gamemode on any games that support it?",
            )
            .map_err(|e| e.to_string())?;

        let mangohud = cli_prompt
            .prompt_confirm(
                "Would you like us to automatically run mangohud on any games that support it?",
            )
            .map_err(|e| e.to_string())?;

        let home = detect_valid_home()?;
        try_save(&home, &Config { gamemode, mangohud })?;
        cli_prompt
            .print_note(format!("Config saved to '{home}'!").as_str())
            .map_err(|e| e.to_string())?;
    }

    cli_prompt
        .outro(format!("Protontweaks setup successfully!").as_str())
        .map_err(|e| e.to_string())?;

    Ok(())
}
