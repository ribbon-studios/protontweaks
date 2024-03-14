use clap::Args;
use futures::future;
use owo_colors::OwoColorize;
use protontweaks_api::Protontweaks;

use crate::{
    apps::{self},
    utils::protontricks,
};

#[derive(Debug, Args)]
pub struct CommandArgs {
    /// The steam launch command '%command%'
    pub app_id: Option<String>,
}

pub async fn command(args: CommandArgs) -> Result<(), String> {
    println!("Automatically applying necessary tweaks...");

    let apps = if let Some(app_id) = args.app_id {
        vec![app_id]
    } else {
        let apps = protontricks::list::try_apps().await?;
        println!(
            "Discovered {} from steam...",
            format!("{} apps", apps.len()).bold()
        );
        apps
    };

    let api = Protontweaks::new();
    let app_ids = api.app_ids().await;

    let tweaked_apps = future::join_all(
        apps.iter()
            .filter(|app| app_ids.contains(app))
            .map(|app_id| api.app(&app_id))
            .collect::<Vec<_>>(),
    )
    .await;

    let (mut tweaks_applied, mut total_tweaks) = (0, 0);

    let results = future::join_all(
        tweaked_apps
            .iter()
            .map(|app| (apps::try_apply(&app)))
            .collect::<Vec<_>>(),
    )
    .await;

    for result in results {
        if let Ok((app_tweaks_applied, app_total_tweaks)) = result {
            tweaks_applied += app_tweaks_applied;
            total_tweaks += app_total_tweaks;
        } else {
            error!("{}", result.unwrap_err());
        }
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
