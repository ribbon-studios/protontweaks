extern crate pretty_env_logger;
#[macro_use]
extern crate log;
use std::{env, str::FromStr};

use clap::{Parser, Subcommand};
use commands::{list, run, service, watch};
use log::LevelFilter;

pub mod apps;
pub mod commands;
pub mod config;
pub mod utils;

static VERSION_LONG: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    // "@",
    // env!("GIT_SHORT_HASH"),
    " (",
    env!("COMPILE_TIME"),
    ")"
);

#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(long_version = VERSION_LONG)]
#[command(args_conflicts_with_subcommands = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[command(flatten)]
    run: run::CommandArgs,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Lists the apps installed on Steam
    List,
    /// Register or Unregister the watch service
    Service(service::CommandArgs),
    /// [experimental]: Runs the steam launch command and applies any necessary tweaks
    Run(run::CommandArgs),
    /// [experimental]: Watches for any steam apps to be installed and automatically adds 'protontweaks' to the launch options
    Watch,
}

fn get_log_level() -> LevelFilter {
    let default_level = env::var("RUST_LOG").unwrap_or_default();
    LevelFilter::from_str(&default_level).unwrap_or(LevelFilter::Warn)
}

#[tokio::main]
async fn main() {
    pretty_env_logger::formatted_builder()
        .filter(None, LevelFilter::Warn)
        .filter(Some("protontweaks"), get_log_level())
        .filter(Some("protontweaks_api"), get_log_level())
        .init();

    let config = config::load();

    let args = Cli::parse();

    let command = args.command.unwrap_or(Commands::Run(args.run));

    let result = match command {
        Commands::List => list::command().await,
        Commands::Service(args) => service::command(args).await,
        Commands::Run(args) => run::command(config, args).await,
        Commands::Watch => watch::command().await,
    };

    if result.is_err() {
        #[allow(unused_must_use)]
        {
            error!("{0}", result.unwrap_err());
        }
    }
}
