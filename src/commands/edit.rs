use std::collections::HashMap;

use clap::Args;
use protontweaks_api::{
    app::{App, TweakSettings, Tweaks},
    system::{GpuDriver, System},
};

use crate::API;

#[derive(Debug, Args)]
pub struct CommandArgs {
    /// The steam launch command '%command%'
    pub app_id: String,
}

pub async fn command(args: CommandArgs) -> Result<(), String> {
    let url = if let Ok(app) = API.try_app(&args.app_id).await {
        format!(
            "https://github.com/ribbon-studios/protontweaks-db/edit/main/apps/{0}.json",
            app.id
        )
    } else {
        let app = App {
            id: args.app_id,
            name: "They Are Billions".to_string(),
            issues: Vec::new(),
            tweaks: Tweaks {
                args: Vec::new(),
                env: HashMap::new(),
                tricks: Vec::new(),
                settings: TweakSettings {
                    gamemode: None,
                    mangohud: None,
                },
                system: System {
                    gpu_driver: GpuDriver {
                        amd: None,
                        nvidia: None,
                    },
                },
            },
        };

        format!(
            "https://github.com/ribbon-studios/protontweaks-db/new/main/apps?filename={0}.json&value={1}",
            &app.id,
            serde_json::to_string_pretty(&app).map_err(|e| e.to_string())?;
        )
    };

    open::that(url).map_err(|e| e.to_string())?;

    Ok(())
}
