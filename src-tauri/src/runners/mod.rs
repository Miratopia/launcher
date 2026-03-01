mod tray;
mod vault;

pub fn setup(mut builder: tauri::Builder<tauri::Wry>) -> tauri::Builder<tauri::Wry> {
    builder = vault::init(builder);

    builder.setup(|app| {
        vault::setup(app)?;
        tray::init(app)?;

        Ok(())
    })
}
