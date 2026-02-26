use tauri::{AppHandle, Manager, Window};

pub fn create_console_window(app_handle: AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("console") {
        let _ = window.show();
        let _ = window.set_focus().ok();
        return Ok(());
    }

    let _window = tauri::WebviewWindowBuilder::new(
        &app_handle,
        "console",
        tauri::WebviewUrl::App("/console".into()),
    )
    .title("Console - Miratopia Launcher")
    .inner_size(1200.0, 700.0)
    .center()
    .build()
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn open_console_window(window: Window) -> Result<(), String> {
    create_console_window(window.app_handle().clone())
}
