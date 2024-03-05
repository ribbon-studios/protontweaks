use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // let git_output = Command::new("git")
    //     .args(["rev-parse", "--short", "HEAD"])
    //     .output()?;
    // let git_short_hash = String::from_utf8(git_output.stdout)?;
    // let git_output = Command::new("git").args(["rev-parse", "HEAD"]).output()?;
    // let git_hash = String::from_utf8(git_output.stdout)?;
    // println!("cargo:rustc-env=GIT_SHORT_HASH={}", git_short_hash);
    // println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    println!(
        "cargo:rustc-env=COMPILE_TIME={}",
        chrono::Utc::now().format("%Y-%m-%d @ %H:%M")
    );
    println!(
        "cargo:rustc-env=PROTONTWEAKS_DB={}",
        "https://tweaks.rains.cafe/v1"
    );
    Ok(())
}
