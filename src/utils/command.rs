use std::{ffi::OsStr, process::Command};

pub fn exec<I, S>(command: &'static str, args: I) -> Result<String, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
    {
        match Command::new(command).args(args).output() {
            Ok(output) => {
                Ok(String::from_utf8(output.stdout).unwrap())
            },
            Err(_) => Err(format!("Failed to call {command}")),
        }
    }

pub fn is_installed(command: &'static str) -> bool {
    exec(command, ["--version"]).is_ok()
}
