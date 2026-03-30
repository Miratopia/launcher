use lighty_launcher::core::AppState;

const LOG_FOLDER_NAME: &str = "logs";

const LATEST_LOG_FILE_NAME: &str = "latest.log";
const LAUNCHER_LOG_FILE_NAME: &str = "launcher.log";

const LOG_TYPE_LATEST: &str = "latest";
const LOG_TYPE_LAUNCHER: &str = "launcher";

#[tauri::command]
pub async fn read_log_file(log_type: String, modpack_id: Option<String>) -> Result<String, String> {
    let dirs = AppState::get_project_dirs();

    let path = match log_type.as_str() {
        LOG_TYPE_LATEST => {
            let id = modpack_id.ok_or(format!("modpack_id is required for {}", LATEST_LOG_FILE_NAME))?;
            dirs.data_dir().join(&id).join(LOG_FOLDER_NAME).join(LATEST_LOG_FILE_NAME)
        }
        LOG_TYPE_LAUNCHER => dirs.data_local_dir().join(LOG_FOLDER_NAME).join(LAUNCHER_LOG_FILE_NAME),
        other => return Err(format!("Unknown log type: {}", other)),
    };

    let bytes = std::fs::read(&path)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}
