extern crate pretty_env_logger;
#[macro_use]
extern crate log;
use std::{env, str::FromStr};

use clap::{Parser, Subcommand};
use log::LevelFilter;

pub mod commands;
pub mod tweaks;
pub mod utils;

static VERSION_LONG: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    "@",
    env!("GIT_SHORT_HASH"),
    " (",
    env!("COMPILE_TIME"),
    ")"
);

#[derive(Parser, Debug)]
#[command(version, about)]
#[command(long_version = VERSION_LONG)]
struct Cli {
    /// The id of the steam app
    app_id: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    // /// Clones repos
    // #[command(arg_required_else_help = true)]
    // Clone {
    //     /// The remote to clone
    //     remote: String,
    // },
    // /// Compare two commits
    // Diff {
    //     #[arg(value_name = "COMMIT")]
    //     base: Option<OsString>,
    //     #[arg(value_name = "COMMIT")]
    //     head: Option<OsString>,
    //     #[arg(last = true)]
    //     path: Option<OsString>,
    //     #[arg(
    //         long,
    //         require_equals = true,
    //         value_name = "WHEN",
    //         num_args = 0..=1,
    //         default_value_t = ColorWhen::Auto,
    //         default_missing_value = "always",
    //         value_enum
    //     )]
    //     color: ColorWhen,
    // },
    // Stash(StashArgs),
    // #[command(external_subcommand)]
    // External(Vec<OsString>),
}

fn get_log_level() -> LevelFilter {
    let default_level = env::var("RUST_LOG").unwrap_or_default();
    LevelFilter::from_str(&default_level).unwrap_or(LevelFilter::Off)
}

fn main() {
    pretty_env_logger::formatted_builder()
        .filter(None, get_log_level())
        .filter(Some("reqwest"), LevelFilter::Warn)
        .init();

    let args = Cli::parse();

    let result = match args.command {
        _ => commands::default::command(),
    };

    if result.is_err() {
        println!("ERROR: {0}", result.unwrap_err());
    }
}
