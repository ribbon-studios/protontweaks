use std::ffi::OsStr;

fn nix_shell<I, S>(args: I) -> Result<String, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    super::command::exec("nix-shell", args)
}

pub fn version() -> Result<String, String> {
    nix_shell(["--version"])
}

pub fn is_installed() -> bool {
    super::command::is_installed("nix-shell")
}
