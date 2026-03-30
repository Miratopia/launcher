use lighty_launcher::core::AppState;

#[tauri::command]
pub async fn read_log_file(log_type: String, modpack_id: Option<String>) -> Result<String, String> {
    let dirs = AppState::get_project_dirs();

    let path = match log_type.as_str() {
        "latest" => {
            let id = modpack_id.ok_or("modpack_id is required for latest.log")?;
            dirs.data_dir().join(&id).join("logs").join("latest.log")
        }
        "launcher" => dirs.data_local_dir().join("logs").join("launcher.log"),
        other => return Err(format!("Unknown log type: {}", other)),
    };

    let bytes = std::fs::read(&path)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}
