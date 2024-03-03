use std::{
    ffi::{OsStr, OsString},
    process::Command,
};

fn nix_shell<I, S>(args: I) -> Result<String, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    super::command::exec("nix-shell", args)
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

pub fn version() -> Result<String, String> {
    nix_shell(["--version"])
}

pub fn is_installed() -> bool {
    super::command::is_installed("nix-shell")
}

#[cfg(test)]
mod tests {
    use crate::utils::nix_shell::{is_installed, version};

    #[test]
    fn version_should_return_the_version() {
        assert_eq!(version().is_ok(), true);
    }

    #[test]
    fn is_installed_should_return_true_if_installed() {
        assert_eq!(is_installed(), true);
    }
}
