// Prevents additional console window on Windows in release, unless the console feature is enabled.
#![cfg_attr(
    all(not(debug_assertions), not(feature = "console")),
    windows_subsystem = "windows"
)]

#[tokio::main]
async fn main() {
    launcher_test_rust_lib::run()
}
