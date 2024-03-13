use std::{path::Path, time::Duration};

use notify::{RecursiveMode, Watcher};
use notify_debouncer_full::new_debouncer;

use crate::utils::steam::Steam;

pub fn command() -> Result<(), String> {
    trace!("Running 'watch' command.");
    let (tx, rx) = std::sync::mpsc::channel();
    let steam = Steam::new();

    trace!("Detected localconfig @ {}.", &steam.localconfig);
    trace!("Updating launch options for startup.");
    let app_ids = steam.app_ids().unwrap_or_default();
    steam.update_all_launch_options(app_ids, map_launch_options());

    trace!("Starting the watch service...");
    let mut debouncer =
        new_debouncer(Duration::from_secs(2), None, tx).map_err(|e| e.to_string())?;

    debouncer
        .watcher()
        .watch(Path::new(&steam.localconfig), RecursiveMode::NonRecursive)
        .unwrap();

    // print all events, non returning
    let mut previous_hash = steam.get_hash();

    trace!("Launch service started!");
    for result in rx {
        match result {
            Ok(_) => {
                info!("Checking for changes...");
                let current_hash = steam.get_hash();

                if previous_hash == current_hash {
                    info!("No changes detected, skipping...");
                } else {
                    info!("Changes detected!");
                    previous_hash = current_hash;

                    info!("Verifying all apps contain the launch options...");
                    let app_ids = steam.app_ids().unwrap_or_default();
                    steam.update_all_launch_options(app_ids, map_launch_options());
                    info!("All apps were updated successfully!");
                }
            }
            Err(error) => log::info!("Error {error:?}"),
        }
    }

    Ok(())
}

fn map_launch_options() -> Box<dyn Fn(Option<String>) -> String> {
    return Box::new(move |launch_options| {
        let launch_options = launch_options.unwrap_or_default();

        let command = "protontweaks %command%";

        if launch_options.contains(command) {
            launch_options
        } else if launch_options == "" {
            command.to_string()
        } else {
            launch_options + " " + command
        }
    });
}
