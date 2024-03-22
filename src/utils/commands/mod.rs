use std::ffi::OsStr;

use async_process::Command;

pub mod protontricks;

pub trait CLI {
    fn exec<I, S>(args: I) -> impl std::future::Future<Output = Result<String, String>>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>;
    fn is_installed() -> impl std::future::Future<Output = bool> + Send;
    fn version() -> impl std::future::Future<Output = Result<String, String>> + Send;
}

pub async fn exec<I, S>(name: &'static str, args: I) -> Result<String, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut command = Command::new(name);

    command.args(args);

    trace!("Running command... {:?}", command);

    match command.output().await {
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
