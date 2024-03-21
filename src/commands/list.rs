use futures::future;

use crate::{utils::commands::protontricks::Protontricks, API};

pub async fn command() -> Result<(), String> {
    println!("Fetching installed apps...");

    let (installed_app_ids, apps) = future::join(Protontricks::apps(), API.apps()).await;

    let apps = apps
        .iter()
        .filter(|app| installed_app_ids.contains(&app.id));

    for app in apps {
        println!("{0} ({1})", app.name, app.id);
    }

    Ok(())
}
