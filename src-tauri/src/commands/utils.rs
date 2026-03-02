use sysinfo::{System, SystemExt};
use tauri::command;

#[command]
pub fn os_total_memory_info() -> u64 {
    let mut sys = System::new();
    sys.refresh_system();

    sys.get_total_memory() / 1024
}
