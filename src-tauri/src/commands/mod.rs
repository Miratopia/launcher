use tauri::{ipc::Invoke, Runtime};

pub mod accounts;
pub mod game;

pub fn handler<R: Runtime>() -> impl Fn(Invoke<R>) -> bool + Send + Sync + 'static {
    tauri::generate_handler![
        game::launch_game,
        game::stop_launch,
        accounts::get_account,
        accounts::add_account,
        accounts::del_account,
        accounts::list_accounts,
        accounts::login_with_microsoft,
        accounts::login_offline,
    ]
}
