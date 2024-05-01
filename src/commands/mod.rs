use clap::Subcommand;

use crate::config::Config;

pub mod info;
pub mod list;
pub mod run;
pub mod setup;
pub mod uninstall;
pub mod watch;

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Lists the apps installed on Steam
    List,
    /// Register or Unregister the watch service
    Setup,
    /// [experimental]: Runs the steam launch command and applies any necessary tweaks
    Run(run::CommandArgs),
    /// [experimental]: Watches for any steam apps to be installed and automatically adds 'protontweaks' to the launch options
    Watch,
    /// Uninstalls the protontweaks service and deletes any configs
    Uninstall,
    /// Outputs information about the system
    Info,
}

pub async fn handle(command: Command) -> Result<(), String> {
    let config = Config::discover();

    match command {
        Command::List => list::command().await,
        Command::Setup => setup::command().await,
        Command::Run(args) => run::command(config, args).await,
        Command::Watch => watch::command().await,
        Command::Uninstall => uninstall::command().await,
        Command::Info => info::command().await,
    }
}
