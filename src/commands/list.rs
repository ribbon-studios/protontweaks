use crate::{
    apps::{self, MiniApp},
    utils::protontricks,
};

pub fn command() -> Result<(), String> {
    println!("Fetching installed apps...");

    let installed_app_ids = protontricks::list::apps()?;

    let apps = apps::apps();

    let apps: Vec<&MiniApp> = apps
        .iter()
        .filter(|app| installed_app_ids.contains(&app.id))
        .collect();

    for app in apps {
        println!("{0} ({1})", app.name, app.id);
    }

    Ok(())
}
