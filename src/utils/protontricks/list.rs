use regex::Regex;

use super::protontricks;

pub fn apps() -> Result<Vec<String>, String> {
    let output = protontricks(["--list"])?;

    let re = Regex::new(r"(?m)^(?<name>[\w\s]+)\s\((?<app_id>\d+)\)$").unwrap();

    let mut results = vec![];

    for caps in re.captures_iter(&output) {
        results.push(caps["app_id"].to_string());
    }

    Ok(results)
}

/// Lists all the installed verbs
pub fn installed(app_id: &str) -> Vec<String> {
    let Ok(output) = protontricks([app_id, "list-installed"]) else {
        return vec![];
    };

    let re = Regex::new(r"(?m)^(?<name>(?:[^-]{2})[-\w]+)$").unwrap();

    let mut results = vec![];

    for caps in re.captures_iter(&output) {
        results.push(caps["name"].to_string())
    }

    results
}
