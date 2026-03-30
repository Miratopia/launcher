use tauri::{AppHandle, Manager, Window};

const CONSOLE_WINDOW_TITLE: &str = "Console - Miratopia Launcher";
const CONSOLE_WINDOW_WIDTH: f64 = 1200.0;
const CONSOLE_WINDOW_HEIGHT: f64 = 700.0;

const CONSOLE_LABEL: &str = "console";
const CONSOLE_WINDOW_URL: &str = "/console";

pub fn create_console_window(app_handle: AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window(CONSOLE_LABEL) {
        let _ = window.show();
        let _ = window.set_focus().ok();
        return Ok(());
    }

    let _window = tauri::WebviewWindowBuilder::new(
        &app_handle,
        CONSOLE_LABEL,
        tauri::WebviewUrl::App(CONSOLE_WINDOW_URL.into()),
    )
    .title(CONSOLE_WINDOW_TITLE)
    .inner_size(CONSOLE_WINDOW_WIDTH, CONSOLE_WINDOW_HEIGHT)
    .center()
    .build()
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn open_console_window(window: Window) -> Result<(), String> {
    create_console_window(window.app_handle().clone())
}
