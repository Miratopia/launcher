use lighty_launcher::core::AppState;
use sysinfo::{System, SystemExt};
use tauri::{command, AppHandle, Manager};

const JRE_DIR_NAME: &str = "jre";

const WINDOW_STATE_CACHE_FILE_NAME: &str = ".window-state";
const WINDOW_STATE_CACHE_FILE_NAME_JSON: &str = ".window-state.json";

#[cfg(target_os = "windows")]
const FOLDER_OPENER: &str = "explorer";
#[cfg(target_os = "macos")]
const FOLDER_OPENER: &str = "open";
#[cfg(target_os = "linux")]
const FOLDER_OPENER: &str = "xdg-open";

#[command]
pub fn os_total_memory_info() -> u64 {
    let mut sys = System::new();
    sys.refresh_system();

    sys.get_total_memory() / 1024
}

#[command]
// Open the launcher folder in the file explorer
pub fn open_launcher_folder() -> Result<(), String> {
    let exe_path = std::env::current_exe()
        .map_err(|e| format!("Failed to get executable path: {}", e))?;
    let folder = exe_path
        .parent()
        .ok_or_else(|| "Failed to resolve launcher directory".to_string())?;

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new(FOLDER_OPENER)
            .arg(folder)
            .spawn()
            .map_err(|e| format!("Failed to open launcher folder: {}", e))?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new(FOLDER_OPENER)
            .arg(folder)
            .spawn()
            .map_err(|e| format!("Failed to open launcher folder: {}", e))?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new(FOLDER_OPENER)
            .arg(folder)
            .spawn()
            .map_err(|e| format!("Failed to open launcher folder: {}", e))?;
    }
    Ok(())
}

/// Remove the window state cache file (window position/size) and downloaded Java runtimes.
/// A launcher restart is necessary for the change to take effect.
#[command]
pub fn clear_cache(app: AppHandle) -> Result<(), String> {
    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to resolve app data dir: {}", e))?;

    let candidates = [WINDOW_STATE_CACHE_FILE_NAME, WINDOW_STATE_CACHE_FILE_NAME_JSON];

    for name in &candidates {
        let path = data_dir.join(name);
        if path.exists() {
            std::fs::remove_file(&path)
                .map_err(|e| format!("Failed to delete {}: {}", name, e))?;
            tracing::info!("Cleared window state cache: {:?}", path);
        }
    }

    let jre_dir = AppState::get_project_dirs().config_dir().join(JRE_DIR_NAME);
    if jre_dir.exists() {
        std::fs::remove_dir_all(&jre_dir)
            .map_err(|e| format!("Failed to delete Java runtimes: {}", e))?;
        tracing::info!("Cleared Java runtimes: {:?}", jre_dir);
    }

    tracing::info!("Restarting launcher after cache clear");
    tauri::process::restart(&app.env());
}
