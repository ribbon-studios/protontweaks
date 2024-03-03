use std::ffi::OsStr;

use super::nix_shell;

pub mod install;
pub mod list;

fn protontricks<I, S>(args: I) -> Result<String, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    if is_installed() {
        return super::command::exec("protontricks", args);
    }

    if nix_shell::is_installed() {
        return nix_shell::run("protontricks", args);
    }

    return Err("Please install 'nix-shell' or 'protontricks'!".to_string());
}

pub fn is_installed() -> bool {
    super::command::is_installed("protontricks")
}

pub fn version() -> Result<String, String> {
    protontricks(["--version"])
}
