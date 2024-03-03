use owo_colors::OwoColorize;

use crate::{
    tweaks::{self, App},
    utils::protontricks,
};

pub fn command(app_id: Option<String>) -> Result<(), String> {
    println!("Automatically applying necessary tweaks...");

    let apps = if let Some(app_id) = app_id {
        vec![app_id]
    } else {
        let apps = protontricks::list::apps()?;
        println!(
            "Discovered {} from steam...",
            format!("{} apps", apps.len()).bold()
        );
        apps
    };

    let tweaks_list = tweaks::list();

    let tweaked_apps = apps
        .iter()
        .filter(|app| tweaks_list.tweaks.contains(app))
        .map(|app_id| tweaks::get(&app_id))
        .collect::<Vec<App>>();

    let (mut tweaks_applied, mut total_tweaks) = (0, 0);

    for app in tweaked_apps {
        let (app_tweaks_applied, app_total_tweaks) = tweaks::apply(&app)?;
        tweaks_applied += app_tweaks_applied;
        total_tweaks += app_total_tweaks;
    }

    if tweaks_applied == 0 {
        println!(
            "{} {}",
            "No tweaks were necessary!".green().bold(),
            format!("({total_tweaks} tweaks attempted)").italic()
        );
    } else {
        println!(
            "Applied {} successfully!",
            format!("{tweaks_applied} tweaks").bold()
        );
    }

    Ok(())
}
