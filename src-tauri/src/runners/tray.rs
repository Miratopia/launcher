use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

const MAIN_WINDOW_LABEL: &str = "main";

const QUIT_ITEM_ID: &str = "quit";
const CONSOLE_ITEM_ID: &str = "console";

pub fn init(app: &tauri::App) -> tauri::Result<()> {
    let quit_i = MenuItem::with_id(app, QUIT_ITEM_ID, "Quitter", true, None::<&str>)?;
    let console_i = MenuItem::with_id(app, CONSOLE_ITEM_ID, "Console", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&console_i, &quit_i])?;

    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .on_tray_icon_event(|tray, event| match event {
            TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } => {
                let app = tray.app_handle();

                if let Some(window) = app.get_webview_window(MAIN_WINDOW_LABEL) {
                    if !window.is_visible().unwrap_or(false) {
                        let _ = window.unminimize();
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
            _ => {}
        })
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id.as_ref() {
            QUIT_ITEM_ID => {
                tracing::info!("🛑 Stopping application !");
                app.exit(0);
            }
            CONSOLE_ITEM_ID => {
                tracing::info!("🔄 Opening console window");
                if let Some(window) = app.get_webview_window(MAIN_WINDOW_LABEL) {
                    let _ = window.show();
                    let _ = window.set_focus().ok();
                }
                let _ = crate::commands::window::create_console_window(app.clone());
            }
            _ => {
                tracing::warn!("⁉️ Menu item <{:?}> not handled", event.id);
            }
        })
        .build(app)?;

    Ok(())
}
