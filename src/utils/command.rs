use std::ffi::OsStr;

use async_process::Command;

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

pub fn join(command_args: Vec<&str>) -> Result<String, String> {
    if command_args.len() == 1 {
        return Ok(command_args[0].to_string());
    }

    let Ok(command) = shlex::try_join(command_args) else {
        return Err("Failed to parse command!".to_string());
    };

    Ok(command)
}

pub fn split(command: &str) -> Result<Vec<String>, String> {
    let Some(command_args) = shlex::split(&command) else {
        return Err("Failed to parse command!".to_string());
    };

    Ok(command_args)
}

#[cfg(test)]
mod tests {
    use crate::utils::command::{join, split};

    fn as_strings(values: Vec<&str>) -> Vec<String> {
        values.iter().map(|v| v.to_string()).collect()
    }

    #[test]
    fn join_tests() {
        assert_eq!(join(vec!["echo 'hello'"]), Ok("echo 'hello'".to_string()));
        assert_eq!(
            join(vec!["echo", "\"this\"", "is", "a", "test"]),
            Ok("echo '\"this\"' is a test".to_string())
        );
        assert_eq!(
            join(vec!["\0", "\0"]),
            Err("Failed to parse command!".to_string())
        );
    }

    #[test]
    fn split_tests() {
        assert_eq!(split("echo 'hello'"), Ok(as_strings(vec!["echo", "hello"])));
        assert_eq!(
            split("echo '\"this\"' is a test"),
            Ok(as_strings(vec!["echo", "\"this\"", "is", "a", "test"]))
        );
        assert_eq!(
            split("echo 'hello"),
            Err("Failed to parse command!".to_string())
        );
    }
}
