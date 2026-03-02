use sysinfo::{System, SystemExt};
use tauri::{command, AppHandle, Manager};

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
        std::process::Command::new("explorer")
            .arg(folder)
            .spawn()
            .map_err(|e| format!("Failed to open launcher folder: {}", e))?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(folder)
            .spawn()
            .map_err(|e| format!("Failed to open launcher folder: {}", e))?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(folder)
            .spawn()
            .map_err(|e| format!("Failed to open launcher folder: {}", e))?;
    }
    Ok(())
}

/// Remove the window state cache file (window position/size)
/// A launcher restart is necessary for the change to take effect.
#[command]
pub fn clear_cache(app: AppHandle) -> Result<(), String> {
    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to resolve app data dir: {}", e))?;

    let candidates = [".window-state", ".window-state.json"];

    for name in &candidates {
        let path = data_dir.join(name);
        if path.exists() {
            std::fs::remove_file(&path)
                .map_err(|e| format!("Failed to delete {}: {}", name, e))?;
            tracing::info!("Cleared window state cache: {:?}", path);
        }
    }

    tracing::info!("Restarting launcher after cache clear");
    tauri::process::restart(&app.env());
}
