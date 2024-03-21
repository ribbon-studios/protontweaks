use owo_colors::OwoColorize;

use crate::{config::wipe, utils::service::unregister};

pub async fn command() -> Result<(), String> {
    println!(
        "{}",
        "~ Thanks for checking out Protontweaks! ~".blue().bold()
    );

    if os_info::get().to_string().contains("NixOS") {
        println!(
            "{} {}",
            "[service] ->",
            "skipping due to read only file system...".italic()
        );
    } else {
        println!("{}", "[service] -> removing...");
        unregister().await?;
        println!("{}", "[service] -> removed successfully!".green());
    }

    println!("{}", "[configs] -> cleaning up...");
    wipe();
    println!("{}", "[configs] -> cleaned up successfully!".green());

    Ok(())
}
