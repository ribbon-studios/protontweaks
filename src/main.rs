extern crate pretty_env_logger;
#[macro_use]
extern crate log;
use std::{env, str::FromStr};

use clap::Parser;
use commands::{run, Command};
use log::LevelFilter;
use protontweaks_api::Protontweaks;

pub mod apps;
pub mod commands;
pub mod config;
pub mod utils;

static VERSION_LONG: &str = concat!(env!("CARGO_PKG_VERSION"), " (", env!("COMPILE_TIME"), ")");

pub const API: Protontweaks = Protontweaks::new_with_url(env!("PROTONTWEAKS_API"));

#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(long_version = VERSION_LONG)]
#[command(args_conflicts_with_subcommands = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,

    #[command(flatten)]
    run: run::CommandArgs,
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

    let args = Cli::parse();

    let command = args.command.unwrap_or(Command::Run(args.run));

    if let Err(error) = commands::handle(command).await {
        error!("{0}", error);
    }
}
