// Prevents additional console window on Windows in release, unless the console feature is enabled.
#![cfg_attr(
    all(not(debug_assertions), not(feature = "console")),
    windows_subsystem = "windows"
)]

use lighty_launcher::{
    core::AppState,
    launch::{init_downloader_config, DownloaderConfig},
};
use tracing_subscriber::prelude::*;

const MAX_RETRIES: u32 = 3;
const INITIAL_DELAY_MS: u64 = 200;
const MAX_CONCURRENT_DOWNLOADS: usize = 16;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _tracing_guard = init_tracing();

    tracing::info!("🏁 Démarrage du launcher");

    const QUALIFIER: &str = "fr";
    const ORGANIZATION: &str = "fr.miratopia.minecraft-launcher";
    const APPLICATION: &str = "";

    let app_state = AppState::new(
        QUALIFIER.to_string(),
        ORGANIZATION.to_string(),
        APPLICATION.to_string(),
    )?;

    init_downloader_config(DownloaderConfig {
        max_concurrent_downloads: MAX_CONCURRENT_DOWNLOADS,
        max_retries: MAX_RETRIES,
        initial_delay_ms: INITIAL_DELAY_MS,
        ..Default::default()
    });

    miratopia_launcher_lib::run(app_state)
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
