use futures::future;
use protontweaks_api::Protontweaks;

use crate::utils::commands::protontricks::Protontricks;

pub async fn command() -> Result<(), String> {
    println!("Fetching installed apps...");
    let api = Protontweaks::new();

    let (installed_app_ids, apps) = future::join(Protontricks::apps(), api.apps()).await;

    let apps = apps
        .iter()
        .filter(|app| installed_app_ids.contains(&app.id));

    for app in apps {
        println!("{0} ({1})", app.name, app.id);
    }

    Ok(())
}
