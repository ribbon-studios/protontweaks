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
    super::command::exec("protontricks", ["--version"]).is_ok()
}

pub fn version() -> Result<String, String> {
    protontricks(["--version"])
}

#[cfg(test)]
mod tests {
    use crate::utils::protontricks::{is_installed, version};

    #[test]
    fn version_should_return_the_version() {
        assert_eq!(version().is_ok(), true);
    }

    #[test]
    fn is_installed_should_return_false_if_not_installed() {
        assert_eq!(is_installed(), true);
    }
}
