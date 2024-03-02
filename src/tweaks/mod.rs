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
    pub dlls: Option<Vec<String>>,
    pub fonts: Option<Vec<String>>,
}

pub fn get_app_url(app_id: &str) -> Url {
    let url = Url::parse(PROTONTWEAKS_DB).unwrap();

    return url.join(&format!("{app_id}.json")).unwrap();
}

pub fn get(app_id: &str) -> Option<App> {
    let url = get_app_url(app_id);

    debug!("Requesting file from '{url}'...");

    let Ok(response) = reqwest::blocking::get(url) else {
        return None;
    };

    trace!("Response received!");

    let Ok(mut app) = response.json::<App>() else {
        return None;
    };

    app.id = app_id.to_string();

    Some(app)
}

pub fn apply(app: &App) -> Result<(u32, u32), String> {
    trace!("App ID: {}; Name: {}", app.id, app.name);
    let mut components: Vec<String> = vec![];

    if let Some(dlls) = &app.tweaks.dlls {
        trace!("DLLs: {}", dlls.len());
        components.append(&mut dlls.clone());
    }

    if let Some(fonts) = &app.tweaks.fonts {
        trace!("Fonts: {}", fonts.len());
        components.append(&mut fonts.clone());
    }

    if components.len() == 0 {
        warn!("No components were found for {} -> {}", app.id, app.name);
        return Ok((0, 0));
    }

    trace!("Installing components for {} -> {}", app.id, app.name);
    return Ok(protontricks::install::components(&app.id, &components)?);
}
