use owo_colors::OwoColorize;

use crate::{
    tweaks::{self, App},
    utils::protontricks,
};

pub fn command() -> Result<(), String> {
    println!("Automatically applying necessary tweaks...");

    let apps = protontricks::list::apps()?;
    let tweaked_apps = apps
        .iter()
        .map(|app_id| tweaks::get(&app_id))
        .filter(|app| app.is_some())
        .map(|app| app.unwrap())
        .collect::<Vec<App>>();

    println!(
        "Discovered {} from steam...",
        format!("{} apps", tweaked_apps.len()).bold()
    );

    let (mut tweaks_applied, mut total_tweaks) = (0, 0);

    for app in tweaked_apps {
        let (app_tweaks_applied, app_total_tweaks) = tweaks::apply(&app)?;
        tweaks_applied += app_tweaks_applied;
        total_tweaks += app_total_tweaks;
    }

    if tweaks_applied == 0 {
        println!(
            "No tweaks were necessary! {}",
            format!("({total_tweaks} tweaks attempted)").bold()
        );
    } else {
        println!(
            "Applied {} successfully!",
            format!("{tweaks_applied} tweaks").bold()
        );
    }

    Ok(())
}
