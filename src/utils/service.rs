use std::{env::current_exe, ffi::OsString};

use clap::Args;
use service_manager::{
    ServiceInstallCtx, ServiceLabel, ServiceManager, ServiceStartCtx, ServiceUninstallCtx,
};

#[derive(Debug, Args)]
pub struct CommandArgs {
    /// Whether to register a systemd service to automatically add launch options to steam
    #[arg(long)]
    pub service: bool,

    /// Disables prompting and sets up based upon the args passed
    #[arg(long)]
    pub no_prompt: bool,
}

const LABEL: &str = "com.protontweaks.watch";

fn get_manager(label: &str) -> Result<(ServiceLabel, Box<dyn ServiceManager>), String> {
    // Get generic service by detecting what is available on the platform
    let manager = <dyn ServiceManager>::native()
        .map_err(|_| "Failed to detect management platform".to_string())?;

    Ok((label.parse().unwrap(), manager))
}

pub async fn register() -> Result<(), String> {
    let (label, manager) = get_manager(LABEL)?;

    manager
        .install(ServiceInstallCtx {
            label: label.clone(),
            program: current_exe().unwrap(),
            args: vec![OsString::from("watch")],
            contents: None,
            username: None,
            working_directory: None,
            environment: None,
        })
        .map_err(|e| e.to_string())?;

    manager
        .start(ServiceStartCtx {
            label: label.clone(),
        })
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn unregister() -> Result<(), String> {
    let (label, manager) = get_manager(LABEL)?;

    if let Err(_) = manager.uninstall(ServiceUninstallCtx {
        label: label.clone(),
    }) {
        return Err("Service does not exist!".to_string());
    }

    Ok(())
}
