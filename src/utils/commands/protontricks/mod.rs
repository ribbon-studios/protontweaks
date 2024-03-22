use std::ffi::OsStr;

use super::CLI;

pub mod install;
pub mod list;
pub struct Protontricks;

impl CLI for Protontricks {
    async fn exec<I, S>(args: I) -> Result<String, String>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        if Self::is_installed().await {
            return super::exec("protontricks", args).await;
        }

        return Err("Please install 'protontricks'!".to_string());
    }

    async fn is_installed() -> bool {
        super::exec("protontricks", ["--version"]).await.is_ok()
    }

    async fn version() -> Result<String, String> {
        Self::exec(["--version"]).await
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::utils::commands::{protontricks::Protontricks, CLI};

    #[tokio::test]
    async fn version_should_return_the_version() {
        assert_eq!(Protontricks::version().await.is_ok(), true);
    }

    #[tokio::test]
    async fn is_installed_should_return_false_if_not_installed() {
        assert_eq!(Protontricks::is_installed().await, !env::var("CI").is_ok());
    }
}
