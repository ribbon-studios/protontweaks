use std::{ffi::OsStr, process::Command};

pub fn exec<I, S>(command: &'static str, args: I) -> Result<String, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    match Command::new(command).args(args).output() {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8(output.stdout).unwrap())
            } else {
                Err(String::from_utf8(output.stderr).unwrap())
            }
        }
        Err(_) => Err(format!("Failed to call {command}")),
    }
}

pub fn is_installed(command: &'static str) -> bool {
    exec(command, ["--version"]).is_ok()
}

pub fn join(command_args: Vec<&str>) -> Result<String, String> {
    if command_args.len() == 1 {
        return Ok(command_args[0].to_string());
    }

    let Ok(command) = shlex::try_join(command_args) else {
        return Err("Failed to parse command!".to_string());
    };

    Ok(command)
}

pub fn split(command: &str) -> Result<Vec<String>, String> {
    let Some(command_args) = shlex::split(&command) else {
        return Err("Failed to parse command!".to_string());
    };

    Ok(command_args)
}
