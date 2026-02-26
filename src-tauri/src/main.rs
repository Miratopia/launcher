// Prevents additional console window on Windows in release, unless the console feature is enabled.
#![cfg_attr(
    all(not(debug_assertions), not(feature = "console")),
    windows_subsystem = "windows"
)]

use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _tracing_guard = init_tracing();

    miratopia_launcher_lib::run();
    Ok(())
}

fn init_tracing() -> tracing_appender::non_blocking::WorkerGuard {
    let log_dir = std::env::temp_dir().join("miratopia-launcher");
    let _ = std::fs::create_dir_all(&log_dir);
    let log_path = log_dir.join("launcher.log");
    let log_file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&log_path)
        .expect("failed to open log file");

    let (non_blocking, guard) = tracing_appender::non_blocking(log_file);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(std::io::stdout)
                .with_ansi(true),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(non_blocking)
                .with_ansi(false),
        )
        .init();

    guard
}
