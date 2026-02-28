use crate::utils::vault::{init_vault_if_needed, VaultState};
use tauri::Manager;

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
    let vault_path = app
        .path()
        .app_data_dir()
        .expect("app_data_dir unavailable")
        .join("vault.hold");

    match init_vault_if_needed(&app.handle(), &vault_state, VAULT_PASSWORD) {
        Ok(_) => {}
        Err(err) => {
            tracing::error!(%err, "vault init failed (1st try)");
            // Si le vault est corrompu, on tente de le supprimer et de réessayer une fois
            if vault_path.exists() {
                match std::fs::remove_file(&vault_path) {
                    Ok(_) => {
                        tracing::warn!("vault.hold corrompu supprimé au démarrage, tentative de réinitialisation");
                        match init_vault_if_needed(&app.handle(), &vault_state, VAULT_PASSWORD) {
                            Ok(_) => {
                                tracing::info!(
                                    "Vault réparé avec succès après suppression au démarrage"
                                );
                            }
                            Err(err2) => {
                                tracing::error!(%err2, "vault init failed après suppression du vault.hold");
                                panic!("Vault irrécupérable : {}", err2);
                            }
                        }
                    }
                    Err(del_err) => {
                        tracing::error!(
                            "Impossible de supprimer vault.hold corrompu au démarrage: {:?}",
                            del_err
                        );
                        panic!("Vault corrompu et impossible à supprimer : {:?}", del_err);
                    }
                }
            } else {
                panic!("Vault irrécupérable au démarrage : {}", err);
            }
        }
    }

    Ok(())
}

/// Initialize the vault state and manage it
pub fn init(builder: tauri::Builder<tauri::Wry>) -> tauri::Builder<tauri::Wry> {
    builder.manage(VaultState::default())
}
