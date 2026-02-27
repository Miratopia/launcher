use tauri::ipc::Invoke;

pub mod accounts;
pub mod game;
pub mod settings;
pub mod window;

pub fn handler() -> impl Fn(Invoke) -> bool + Send + Sync + 'static {
    tauri::generate_handler![
        window::open_console_window,
        game::launch_game,
        game::stop_launch,
        accounts::get_account,
        accounts::add_account,
        accounts::del_account,
        accounts::list_accounts,
        settings::display_settings,
        settings::update_settings,
    ]
}
