use std::ffi::OsStr;

use futures::future;

use super::nix_shell;

pub mod install;
pub mod list;

async fn protontricks<I, S>(args: I) -> Result<String, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let (tricks_installed, nix_shell_installed) =
        future::join(is_installed(), nix_shell::is_installed()).await;

    if tricks_installed {
        return super::command::exec("protontricks", args).await;
    }

    if nix_shell_installed {
        return nix_shell::run("protontricks", args);
    }

    return Err("Please install 'nix-shell' or 'protontricks'!".to_string());
}

pub async fn is_installed() -> bool {
    super::command::exec("protontricks", ["--version"])
        .await
        .is_ok()
}

pub async fn version() -> Result<String, String> {
    protontricks(["--version"]).await
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::utils::protontricks::{is_installed, version};

    #[tokio::test]
    async fn version_should_return_the_version() {
        assert_eq!(version().await.is_ok(), true);
    }

    #[tokio::test]
    async fn is_installed_should_return_false_if_not_installed() {
        assert_eq!(is_installed().await, !env::var("CI").is_ok());
    }
}
