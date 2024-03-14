use std::{
    ffi::{OsStr, OsString},
    process::Command,
};

async fn nix_shell<I, S>(args: I) -> Result<String, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    super::command::exec("nix-shell", args).await
}

pub fn run<I, S>(program: &str, args: I) -> Result<String, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut run_args: Vec<String> = vec![program.to_string()];
    for arg in args {
        let arg_os_str: OsString = arg.as_ref().to_owned();
        run_args.push(arg_os_str.to_str().unwrap().to_owned());
    }

    let output = Command::new("nix-shell")
        .env("NIXPKGS_ALLOW_UNFREE", "1")
        .args(["--impure", "-p", program, "--run", &run_args.join(" ")])
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8(output.stdout).unwrap())
            } else {
                Err(String::from_utf8(output.stderr).unwrap())
            }
        }
        Err(_) => Err("Failed to call nix-shell".to_string()),
    }
}

pub async fn version() -> Result<String, String> {
    nix_shell(["--version"]).await
}

pub async fn is_installed() -> bool {
    super::command::exec("nix-shell", ["--version"])
        .await
        .is_ok()
}

#[cfg(test)]
mod tests {
    use crate::utils::nix_shell::{is_installed, version};

    #[tokio::test]
    async fn version_should_return_the_version() {
        assert_eq!(version().await.is_ok(), true);
    }

    #[tokio::test]
    async fn is_installed_should_return_true_if_installed() {
        assert_eq!(is_installed().await, true);
    }
}
