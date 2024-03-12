use owo_colors::OwoColorize;
use std::{collections::HashMap, process::Command};

use clap::Args;
use regex::Regex;

use crate::{
    apps::{self, App},
    utils::{command, env},
};

#[derive(Debug, Args)]
pub struct CommandArgs {
    /// The steam launch command '%command%'
    pub command_args: Option<Vec<String>>,
}

pub fn command(args: CommandArgs) -> Result<(), String> {
    let (command, args, app) = parse_command(args)?;

    if let Some(app) = &app {
        let (app_tweaks_applied, app_total_tweaks) = apps::apply_safe(app);

        if app_tweaks_applied == 0 {
            println!(
                "{} {}",
                "No tweaks were necessary!".green().bold(),
                format!("({app_total_tweaks} tweaks attempted)").italic()
            );
        } else {
            println!(
                "Applied {} successfully!",
                format!("{app_tweaks_applied} tweaks").bold()
            );
        }
    }

    let env = &app.map_or(HashMap::new(), |app| app.tweaks.env);

    Command::new(command)
        .args(args)
        .envs(env)
        .spawn()
        .expect("Failed to run command")
        .wait()
        .expect("Failed to wait for command");

    Ok(())
}

fn parse_command(args: CommandArgs) -> Result<(String, Vec<String>, Option<App>), String> {
    let command_args = args.command_args.unwrap();
    let command_args: Vec<&str> = command_args.iter().map(|x| x.as_str()).collect();
    let command = command::join(command_args)?;

    let re = Regex::new(r"AppId=(?<app_id>\d+)").unwrap();

    if let Some(caps) = re.captures(&command) {
        let app_id = &caps["app_id"];

        println!("App ID: {0}", &caps["app_id"]);

        let mut app = apps::get(app_id);

        let command = command::split(&command)?;

        if let Some(esync) = app.tweaks.settings.esync {
            app.tweaks.env.insert(
                "PROTON_NO_ESYNC".to_string(),
                if esync {
                    "1".to_string()
                } else {
                    "0".to_string()
                },
            );
        }

        if let Some(fsync) = app.tweaks.settings.fsync {
            app.tweaks
                .env
                .insert("PROTON_NO_FSYNC".to_string(), env::convert_bool(fsync));
        }

        return Ok((command[0].clone(), command[1..].to_vec(), Some(app)));
    }

    warn!("Protontweaks purely acts as a passthrough for non-steam games!");
    let command = command::split(&command)?;

    return Ok((command[0].clone(), command[1..].to_vec(), None));
}

#[cfg(test)]
pub mod tests {
    use super::{parse_command, CommandArgs};

    #[test]
    pub fn parse_command_should_support_simple_commands() {
        let (command, args, app) = parse_command(CommandArgs {
            command_args: Some(vec!["echo".to_string(), "hello".to_string()]),
        })
        .expect("Failed to parse command.");

        assert_eq!(command, "echo");
        assert_eq!(args, vec!["hello"]);
        assert!(app.is_none(), "Expected app to not be defined!");
    }

    #[test]
    pub fn parse_command_should_support_unified_commands() {
        let (command, args, app) = parse_command(CommandArgs {
            command_args: Some(vec!["echo hello".to_string()]),
        })
        .expect("Failed to execute command.");

        assert_eq!(command, "echo");
        assert_eq!(args, vec!["hello"]);
        assert!(app.is_none(), "Expected app to not be defined!");
    }

    #[test]
    pub fn parse_command_should_support_steam_launch_commands() {
        let command_args = vec![
            "~/.local/share/Steam/ubuntu12_32/reaper",
            "SteamLaunch",
            "AppId=644930",
            "--",
            "/home/ceci/.local/share/Steam/ubuntu12_32/steam-launch-wrapper",
            "--",
            "'/home/ceci/.local/share/Steam/steamapps/common/SteamLinuxRuntime_sniper'/_v2-entry-point",
            "--verb=waitforexitandrun",
            "--",
            "'/home/ceci/.local/share/Steam/steamapps/common/Proton 9.0 (Beta)'/proton",
            "waitforexitandrun",
            "'/home/ceci/.local/share/Steam/steamapps/common/They Are Billions/TheyAreBillions.exe'"
        ].iter_mut().map(|x| x.to_string()).collect::<Vec<String>>();

        let (command, args, app) = parse_command(CommandArgs {
            command_args: Some(command_args),
        })
        .expect("Failed to execute command.");

        assert_eq!(command, "~/.local/share/Steam/ubuntu12_32/reaper");
        assert_eq!(args.len(), 11);

        let app = app.unwrap();

        assert_eq!(app.tweaks.env.len(), 0);
        assert_eq!(app.tweaks.tricks.len(), 1);
    }
}
