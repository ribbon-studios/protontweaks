use std::ffi::OsStr;

use super::nix_shell;

pub mod install;
pub mod list;

fn protontricks<I, S>(args: I) -> Result<String, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    if !is_installed() {
        if !nix_shell::is_installed() {
            return Err("Please install 'nix-shell' or 'protontricks'!".to_string());
        }
    }

    super::command::exec("protontricks", args)
}

pub fn is_installed() -> bool {
    super::command::is_installed("nix-shell")
}

pub fn version() -> Result<String, String> {
    protontricks(["--version"])
}
