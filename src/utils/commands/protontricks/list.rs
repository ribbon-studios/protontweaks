use regex::Regex;

use crate::utils::commands::CLI;

use super::Protontricks;

impl Protontricks {
    pub async fn try_apps() -> Result<Vec<String>, String> {
        let output = Protontricks::exec(["--list"]).await?;

        let re = Regex::new(r"(?m)^(?<name>[\w\s]+)\s\((?<app_id>\d+)\)$").unwrap();

        let mut results = vec![];

        for caps in re.captures_iter(&output) {
            results.push(caps["app_id"].to_string());
        }

        Ok(results)
    }

    pub async fn apps() -> Vec<String> {
        Self::try_apps().await.unwrap()
    }

    /// Lists all the installed verbs
    pub async fn installed(app_id: &str) -> Vec<String> {
        let Ok(output) = Protontricks::exec([app_id, "list-installed"]).await else {
            return vec![];
        };

        let re = Regex::new(r"(?m)^(?<name>(?:[^-]{2})[-\w]+)$").unwrap();

        let mut results = vec![];

        for caps in re.captures_iter(&output) {
            results.push(caps["name"].to_string())
        }

        results
    }
}
