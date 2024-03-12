use std::{ffi::OsStr, process::Command};

pub fn exec<I, S>(name: &'static str, args: I) -> Result<String, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut command = Command::new(name);

    command.args(args);

    trace!("Running command... {:?}", command);

    match command.output() {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8(output.stdout).unwrap())
            } else {
                Err(String::from_utf8(output.stderr).unwrap())
            }
        }
        Err(_) => Err(format!("Failed to call {name}")),
    }
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
