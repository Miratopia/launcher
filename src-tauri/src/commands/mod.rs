use tauri::ipc::Invoke;

pub mod accounts;
pub mod modpacks;
pub mod settings;
pub mod window;

pub fn handler() -> impl Fn(Invoke) -> bool + Send + Sync + 'static {
    tauri::generate_handler![
        window::open_console_window,
        modpacks::start_modpack,
        modpacks::stop_modpack,
        accounts::get_account,
        accounts::add_account,
        accounts::del_account,
        accounts::list_accounts,
        settings::display_modpack_settings,
        settings::update_modpack_settings,
    ]
}
