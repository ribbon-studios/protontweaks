pub async fn is_installed() -> bool {
    super::command::exec("mangohud", ["--version"])
        .await
        .is_ok()
}
