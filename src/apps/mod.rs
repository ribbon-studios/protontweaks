use protontweaks_api::app::App;

use crate::utils::protontricks;

/// Applies the app and returns a result of whether it was successful
pub async fn try_apply(app: &App) -> Result<(u32, u32), String> {
    let tweaks = app.flatten().await;
    trace!("App ID: {}; Name: {}", app.id, app.name);

    if tweaks.tricks.len() == 0 {
        warn!("No tricks were found for {} -> {}", app.id, app.name);
        return Ok((0, 0));
    }

    trace!("Installing tricks for {} -> {}", app.id, app.name);
    let tweaks_applied = protontricks::install::components(&app.id, &tweaks.tricks).await?;

    return Ok((tweaks_applied, tweaks.tricks.len().try_into().unwrap()));
}

/// Applies the app and panics if a failure occurs.
pub async fn apply(app: &App) -> (u32, u32) {
    return try_apply(app).await.unwrap();
}

/// Applies the app, if an error occurs it simply logs it and returns that no tweaks were applied
pub async fn apply_safe(app: &App) -> (u32, u32) {
    match try_apply(app).await {
        Ok(result) => result,
        Err(e) => {
            error!("{e}");
            (0, app.tweaks.tricks.len().try_into().unwrap())
        }
    }
}
