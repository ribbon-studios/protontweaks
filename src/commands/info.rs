use owo_colors::OwoColorize;
use protontweaks_api::system::{get_system_info, SystemInfo, GPU};

pub async fn command() -> Result<(), String> {
    let system_info = get_system_info().await.unwrap_or(SystemInfo {
        driver: "Unknown".to_string(),
        driver_type: GPU::UNKNOWN,
    });

    println!("Driver: {}", system_info.driver.italic());
    println!("Driver Type: {}", system_info.driver_type.as_str().italic());
    Ok(())
}
