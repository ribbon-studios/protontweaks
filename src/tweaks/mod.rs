use std::collections::HashMap;

use reqwest::Url;
use serde::Deserialize;

use crate::utils::protontricks;

const PROTONTWEAKS_DB: &str = env!("PROTONTWEAKS_DB");

#[derive(Debug, Deserialize)]
pub struct App {
    #[serde(skip_deserializing)]
    pub id: String,
    pub name: String,
    pub tweaks: Tweaks,
}

#[derive(Debug, Deserialize)]
pub struct Tweaks {
    pub tricks: Vec<String>,
    pub env: HashMap<String, String>,
    pub settings: TweakSettings,
}

#[derive(Debug, Deserialize)]
pub struct TweakSettings {
    pub esync: Option<bool>,
    pub fsync: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct TweaksList {
    pub sha: String,
    pub short_sha: String,
    pub tweaks: Vec<String>,
}

pub fn url(path: &str) -> Url {
    let url = Url::parse(PROTONTWEAKS_DB).unwrap();

    return url.join(&format!("{path}.json")).unwrap();
}

pub fn list() -> TweaksList {
    let url = url("tweaks");

    debug!("Requesting tweaks from '{url}'...");

    let response = reqwest::blocking::get(url).unwrap();

    trace!("Response received!");

    response.json::<TweaksList>().unwrap()
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
