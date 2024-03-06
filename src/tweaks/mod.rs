use std::collections::HashMap;

use reqwest::Url;
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
pub struct Issue {
    pub description: String,
    pub solution: String,
}

#[derive(Debug, Deserialize)]
pub struct Tweaks {
    pub tricks: Vec<String>,
    pub env: HashMap<String, String>,
    pub settings: TweakSettings,
}

#[derive(Debug, Deserialize)]
pub struct MiniApp {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct TweakSettings {
    pub esync: Option<bool>,
    pub fsync: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct AppsList {
    pub sha: String,
    pub short_sha: String,
    pub tweaks: Vec<MiniApp>,
}

pub fn url(path: &str) -> Url {
    let url = Url::parse(PROTONTWEAKS_DB).unwrap();

    return url.join(&format!("{path}.json")).unwrap();
}

pub fn apps() -> AppsList {
    let url = url("tweaks");

    debug!("Requesting tweaks from '{url}'...");

    let response = reqwest::blocking::get(url).unwrap();

    trace!("Response received!");

    response.json::<AppsList>().unwrap()
}

pub fn app_ids() -> Vec<String> {
    apps().tweaks.iter().map(|x| x.id.to_owned()).collect()
}

pub fn get(app_id: &str) -> App {
    let url = url(app_id);

    debug!("Requesting file from '{url}'...");

    let response =
        reqwest::blocking::get(url).expect(&format!("Failed to get tweaks for {}", app_id));

    trace!("Response received!");

    let mut app = response
        .json::<App>()
        .expect(&format!("Failed to parse tweak data for {}", app_id));

    app.id = app_id.to_string();

    app
}

pub fn apply(app: &App) -> Result<(u32, u32), String> {
    trace!("App ID: {}; Name: {}", app.id, app.name);

    if app.tweaks.tricks.len() == 0 {
        warn!("No tricks were found for {} -> {}", app.id, app.name);
        return Ok((0, 0));
    }

    trace!("Installing tricks for {} -> {}", app.id, app.name);
    return Ok(protontricks::install::components(
        &app.id,
        &app.tweaks.tricks,
    )?);
}

#[cfg(test)]
mod tests {
    use crate::tweaks::{app_ids, apps, get, url};

    #[test]
    fn url_should_create_a_tweaks_url() {
        assert_eq!(
            url("tweaks").to_string(),
            "https://api.protontweaks.com/tweaks.json"
        );
    }

    #[test]
    fn apps_should_return_the_tweaks_list() {
        let apps = apps();

        assert!(
            apps.tweaks.len() > 0,
            "Expected to receive a list of valid tweaked apps!"
        );
    }

    #[test]
    fn app_ids_should_return_the_tweaks_list() {
        let ids = app_ids();

        assert!(
            ids.len() > 0,
            "Expected to receive a list of valid tweaked apps!"
        );
    }

    #[test]
    fn get_should_return_the_tweak_info() {
        let expected_id = "644930";
        let tweak = get(expected_id);

        assert_eq!(tweak.id, expected_id);
        assert_eq!(tweak.issues.len(), 1);
        assert_eq!(tweak.tweaks.tricks.len(), 1);
        assert_eq!(tweak.tweaks.env.len(), 0);
        assert_eq!(tweak.tweaks.settings.esync, None);
        assert_eq!(tweak.tweaks.settings.fsync, None);
    }
}
