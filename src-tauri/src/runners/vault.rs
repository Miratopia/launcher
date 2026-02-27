use tauri::Manager;
use crate::commands::accounts::{init_vault_if_needed, VaultState};

const VAULT_PASSWORD: &str = "dev-vault-password";

/// Setup the vault plugin and initialize the vault if needed
pub fn setup(app: &tauri::App) -> tauri::Result<()> {
    let salt_path = app
        .path()
        .app_local_data_dir()
        .expect("could not resolve app local data path")
        .join("salt.txt");

    app.handle()
        .plugin(tauri_plugin_stronghold::Builder::with_argon2(&salt_path).build())?;

    let vault_state = app.state::<VaultState>();
    if let Err(err) = init_vault_if_needed(&app.handle(), &vault_state, VAULT_PASSWORD) {
        tracing::error!(%err, "vault init failed");
    }

    Ok(())
}

/// Initialize the vault state and manage it
pub fn init(builder: tauri::Builder<tauri::Wry>) -> tauri::Builder<tauri::Wry> {
    builder.manage(VaultState::default())
}
