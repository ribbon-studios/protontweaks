use futures::future;
use protontweaks_api::Protontweaks;

use crate::utils::protontricks;

pub async fn command() -> Result<(), String> {
    println!("Fetching installed apps...");
    let api = Protontweaks::new();

    let (installed_app_ids, apps) = future::join(protontricks::list::apps(), api.apps()).await;

    let apps = apps
        .iter()
        .filter(|app| installed_app_ids.contains(&app.id));

    for app in apps {
        println!("{0} ({1})", app.name, app.id);
    }

    Ok(())
}
