// Prevents additional console window on Windows in release, unless the console feature is enabled.
#![cfg_attr(
    all(not(debug_assertions), not(feature = "console")),
    windows_subsystem = "windows"
)]

use lighty_launcher::{
    core::AppState,
    launch::{init_downloader_config, DownloaderConfig},
};
use tracing_subscriber::{prelude::*, EnvFilter};

const MAX_RETRIES: u32 = 3;
const INITIAL_DELAY_MS: u64 = 200;
const MAX_CONCURRENT_DOWNLOADS: usize = 16;

const LOG_FOLDER_NAME: &str = "logs";
const LOG_FILE_NAME: &str = "launcher.log";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    const QUALIFIER: &str = "fr";
    const PRODUCT_NAME: &str = env!("TAURI_PRODUCT_NAME");

    let app_state = AppState::new(
        QUALIFIER.to_string(),
        PRODUCT_NAME.to_string(),
        "".to_string(),
    )?;

    let _tracing_guard = init_tracing(&app_state);
    tracing::info!("🏁 Démarrage du launcher");

    init_downloader_config(DownloaderConfig {
        max_concurrent_downloads: MAX_CONCURRENT_DOWNLOADS,
        max_retries: MAX_RETRIES,
        initial_delay_ms: INITIAL_DELAY_MS,
        ..Default::default()
    });

    miratopia_launcher_lib::run(app_state)
}

fn init_tracing(_app_state: &AppState) -> tracing_appender::non_blocking::WorkerGuard {
    let roaming_app_dir = AppState::get_project_dirs()
        .data_local_dir()
        .to_path_buf();

    let log_dir = roaming_app_dir.join(LOG_FOLDER_NAME);
    let _ = std::fs::create_dir_all(&log_dir);
    let log_path = log_dir.join(LOG_FILE_NAME);
    let log_file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&log_path)
        .expect("failed to open log file");

    let (non_blocking, guard) = tracing_appender::non_blocking(log_file);

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false);

    let default_filter = if cfg!(debug_assertions) {
        "debug"
    } else {
        "info"
    };
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(default_filter));

    #[cfg(debug_assertions)]
    tracing_subscriber::registry()
        .with(env_filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(std::io::stdout)
                .with_ansi(true),
        )
        .with(file_layer)
        .init();

    #[cfg(not(debug_assertions))]
    tracing_subscriber::registry()
        .with(env_filter)
        .with(file_layer)
        .init();

    guard
}
