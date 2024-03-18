use std::ffi::OsString;

pub async fn is_installed() -> bool {
    super::command::exec::<Vec<OsString>, OsString>("gamemoderun", vec![])
        .await
        .is_ok()
}
