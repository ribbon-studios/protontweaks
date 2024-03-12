use std::collections::HashMap;

use reqwest::{StatusCode, Url};
use serde::Deserialize;

use crate::utils::protontricks;

const PROTONTWEAKS_DB: &str = env!("PROTONTWEAKS_DB");

#[derive(Debug, Deserialize)]
pub struct App {
    pub id: String,
    pub name: String,
    pub tweaks: Tweaks,
    pub issues: Vec<Issue>,
}

#[derive(Debug, Deserialize)]
pub struct Tweaks {
    pub tricks: Vec<String>,
    pub env: HashMap<String, String>,
    pub settings: TweakSettings,
}

#[derive(Debug, Deserialize)]
pub struct TweakSettings {
    pub gamemode: Option<bool>,
    pub mangohud: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Issue {
    pub description: String,
    pub solution: String,
}

#[derive(Debug, Deserialize)]
pub struct AppsList {
    pub sha: String,
    pub short_sha: String,
    pub apps: Vec<MiniApp>,
}

#[derive(Debug, Deserialize)]
pub struct MiniApp {
    pub id: String,
    pub name: String,
}

pub fn url(path: &str) -> Url {
    let url = Url::parse(PROTONTWEAKS_DB).unwrap();

    return url.join(&format!("{path}.json")).unwrap();
}

pub fn apps_list() -> AppsList {
    let url = url("apps");

    debug!("Requesting apps from '{url}'...");

    let response = reqwest::blocking::get(url).unwrap();

    trace!("Apps received!");

    response.json::<AppsList>().unwrap()
}

pub fn apps() -> Vec<MiniApp> {
    apps_list().apps
}

pub fn list_ids() -> Vec<String> {
    apps_list().apps.iter().map(|x| x.id.to_owned()).collect()
}

pub fn try_get(app_id: &str) -> Result<App, String> {
    let url = url(app_id);

    debug!("Requesting file from '{url}'...");

    let response = reqwest::blocking::get(url).map_err(|e| {
        warn!("{}", e.to_string());
        e.to_string()
    })?;

    if response.status() == StatusCode::NOT_FOUND {
        let message = format!("App {app_id} does not exist!");
        info!("{message}");
        return Err(message);
    }

    trace!("App received!");

    let mut app = response
        .json::<App>()
        .map_err(|_| "Failed to parse app info".to_string())?;

    app.id = app_id.to_string();

    Ok(app)
}

pub fn get(app_id: &str) -> App {
    try_get(app_id).unwrap()
}

/// Applies the app and returns a result of whether it was successful
pub fn try_apply(app: &App) -> Result<(u32, u32), String> {
    trace!("App ID: {}; Name: {}", app.id, app.name);

    if app.tweaks.tricks.len() == 0 {
        warn!("No tricks were found for {} -> {}", app.id, app.name);
        return Ok((0, 0));
    }

    trace!("Installing tricks for {} -> {}", app.id, app.name);
    let tweaks_applied = protontricks::install::components(&app.id, &app.tweaks.tricks)?;

    return Ok((tweaks_applied, app.tweaks.tricks.len().try_into().unwrap()));
}

/// Applies the app and panics if a failure occurs.
pub fn apply(app: &App) -> (u32, u32) {
    return try_apply(app).unwrap();
}

/// Applies the app, if an error occurs it simply logs it and returns that no tweaks were applied
pub fn apply_safe(app: &App) -> (u32, u32) {
    match try_apply(app) {
        Ok(result) => result,
        Err(e) => {
            error!("{e}");
            (0, app.tweaks.tricks.len().try_into().unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::apps::{apps_list, get, list_ids, url};

    #[test]
    fn url_should_create_a_apps_url() {
        assert_eq!(
            url("apps").to_string(),
            "https://api.protontweaks.com/apps.json"
        );
    }

    #[test]
    fn apps_list_should_return_the_tweaks_list() {
        let apps_list = apps_list();

        assert!(
            apps_list.apps.len() > 0,
            "Expected to receive a list of valid tweaked apps!"
        );
    }

    #[test]
    fn list_ids_should_return_the_tweaks_list() {
        let ids = list_ids();

        assert!(
            ids.len() > 0,
            "Expected to receive a list of valid tweaked apps!"
        );
    }

    #[test]
    fn get_should_return_the_app_info() {
        let expected_id = "644930";
        let app = get(expected_id);

        assert_eq!(app.id, expected_id);
        assert_eq!(app.issues.len(), 1);
        assert_eq!(app.tweaks.tricks.len(), 1);
        assert_eq!(app.tweaks.env.len(), 0);
        assert_eq!(app.tweaks.settings.gamemode, None);
        assert_eq!(app.tweaks.settings.mangohud, None);
    }
}
