mod tray;
mod vault;

pub fn setup(builder: tauri::Builder<tauri::Wry>) -> tauri::Builder<tauri::Wry> {
    builder.setup(|app| {
        vault::init(app)?;
        tray::init(app)?;

        Ok(())
    })
}
