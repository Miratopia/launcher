use iota_stronghold::{KeyProvider, SnapshotPath};
use std::sync::{Arc, Mutex};
use tauri::AppHandle;
use tauri::Manager;
use tauri_plugin_stronghold::{kdf::KeyDerivation, stronghold::Stronghold};

pub struct VaultState {
    // On garde une instance en mémoire (ouverte) pour éviter de recharger à chaque commande
    pub inner: Mutex<Option<Stronghold>>,
    pub app_handle: Mutex<Option<Arc<AppHandle>>>,
    pub password: Mutex<Option<String>>,
}

impl Default for VaultState {
    fn default() -> Self {
        Self {
            inner: Mutex::new(None),
            app_handle: Mutex::new(None),
            password: Mutex::new(None),
        }
    }
}

pub fn init_vault_if_needed(
    app: &AppHandle,
    state: &VaultState,
    password: &str,
) -> Result<(), String> {
    let mut guard = state.inner.lock().unwrap();
    if guard.is_some() {
        return Ok(());
    }

    // Toujours enregistrer le contexte pour permettre l'auto-réparation
    *state.app_handle.lock().unwrap() = Some(Arc::new(app.clone()));
    *state.password.lock().unwrap() = Some(password.to_string());

    let v_path = vault_path(app);
    tracing::info!("Vault path: {:?}", v_path);

    let key = KeyDerivation::argon2(password, &salt_path(app));
    let key_provider = match KeyProvider::try_from(zeroize::Zeroizing::new(key.clone())) {
        Ok(kp) => kp,
        Err(e) => return Err(format!("Failed to create KeyProvider: {:?}", e)),
    };

    let mut sh = match Stronghold::new(v_path.clone(), key.clone()) {
        Ok(s) => s,
        Err(e) => return Err(e.to_string()),
    };

    if v_path.exists() {
        tracing::info!("Loading existing snapshot from {:?}", v_path);
        let snapshot_path = SnapshotPath::from_path(&v_path);
        match sh.load_snapshot(&key_provider, &snapshot_path) {
            Ok(_) => {
                tracing::info!("Successfully loaded snapshot");
            }
            Err(e) => {
                tracing::warn!("Failed to load existing snapshot: {:?}", e);
                tracing::warn!(
                    "Tentative de suppression du vault.hold corrompu: {:?}",
                    v_path
                );
                match std::fs::remove_file(&v_path) {
                    Ok(_) => {
                        tracing::info!("vault.hold corrompu supprimé automatiquement");
                        // On tente de recréer un nouveau Stronghold
                        match Stronghold::new(v_path.clone(), key.clone()) {
                            Ok(new_sh) => {
                                // On force la réinitialisation complète du state
                                *guard = None;
                                *state.app_handle.lock().unwrap() = None;
                                *state.password.lock().unwrap() = None;
                                sh = new_sh;
                                tracing::info!(
                                    "Nouveau vault.hold créé après suppression du corrompu"
                                );
                                // Réenregistrer le contexte après reset
                                *state.app_handle.lock().unwrap() = Some(Arc::new(app.clone()));
                                *state.password.lock().unwrap() = Some(password.to_string());
                            }
                            Err(e) => {
                                tracing::error!(
                                    "Vault corrompu supprimé mais impossible de recréer: {}",
                                    e
                                );
                                return Err(format!(
                                    "Vault corrompu supprimé mais impossible de recréer: {}",
                                    e
                                ));
                            }
                        }
                    }
                    Err(del_err) => {
                        tracing::error!(
                            "Impossible de supprimer vault.hold corrompu: {:?}",
                            del_err
                        );
                        return Err(format!(
                            "Vault corrompu et impossible à supprimer: {:?}",
                            del_err
                        ));
                    }
                }
            }
        }
    } else {
        tracing::info!("No existing snapshot found, creating new vault");
    }

    *guard = Some(sh);
    Ok(())
}

pub fn commit_snapshot(state: &VaultState) -> Result<(), String> {
    let guard = state.inner.lock().unwrap();
    let sh = guard.as_ref().ok_or("Vault not initialized")?;

    let app_guard = state.app_handle.lock().unwrap();
    let app = app_guard.as_ref().ok_or("App handle not set")?;

    let pwd_guard = state.password.lock().unwrap();
    let password = pwd_guard.as_ref().ok_or("Password not set")?;

    let v_path = vault_path(app);
    tracing::info!("Committing snapshot to: {:?}", v_path);
    let snapshot_path = SnapshotPath::from_path(&v_path);
    let key = KeyDerivation::argon2(password, &salt_path(app));
    let key_provider = KeyProvider::try_from(zeroize::Zeroizing::new(key))
        .map_err(|e| format!("Failed to create KeyProvider: {:?}", e))?;

    sh.commit_with_keyprovider(&snapshot_path, &key_provider)
        .map_err(|e| {
            let err_msg = format!("Failed to commit snapshot: {:?}", e);
            tracing::error!("{}", err_msg);
            err_msg
        })?;

    tracing::info!("Snapshot committed successfully to {:?}", v_path);
    Ok(())
}

pub fn with_sh<F, R>(state: &VaultState, f: F) -> Result<R, String>
where
    F: FnOnce(&Stronghold) -> Result<R, String>,
{
    let mut guard = state.inner.lock().unwrap();
    if guard.is_none() {
        // Tentative d'auto-réparation du vault
        let app_handle = state.app_handle.lock().unwrap();
        let password = state.password.lock().unwrap();
        if let (Some(app), Some(pwd)) = (app_handle.as_ref(), password.as_ref()) {
            drop(guard);
            match init_vault_if_needed(app, state, pwd) {
                Ok(_) => {
                    guard = state.inner.lock().unwrap();
                }
                Err(e) => {
                    return Err(format!("Vault corrompu et réparation impossible : {}. Veuillez relancer le launcher ou supprimer manuellement le fichier vault.hold.", e));
                }
            }
        } else {
            return Err("Vault non initialisé et impossible de récupérer le contexte d'initialisation. Veuillez relancer le launcher.".to_string());
        }
    }
    let sh = guard
        .as_ref()
        .ok_or("Vault not initialized (auto-repair failed)")?;
    f(sh)
}

fn vault_path(app: &AppHandle) -> std::path::PathBuf {
    app.path()
        .app_data_dir()
        .expect("app_data_dir unavailable")
        .join("vault.hold")
}

fn salt_path(app: &AppHandle) -> std::path::PathBuf {
    app.path()
        .app_local_data_dir()
        .expect("app_local_data_dir unavailable")
        .join("salt.txt")
}
