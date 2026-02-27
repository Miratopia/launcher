use chrono::{DateTime, Utc};
use lighty_auth::{microsoft::MicrosoftRefresh, offline::OfflineRefresh, AuthProvider};
use lighty_launcher::auth::{MicrosoftAuth, OfflineAuth};
use lighty_launcher::event::EventBus;
use lighty_launcher::{Authenticator, UserProfile};

use iota_stronghold::{KeyProvider, SnapshotPath};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager, State};
use tauri_plugin_stronghold::{kdf::KeyDerivation, stronghold::Stronghold};

pub struct VaultState {
    // On garde une instance en mémoire (ouverte) pour éviter de recharger à chaque commande
    inner: Mutex<Option<Stronghold>>,
    app_handle: Mutex<Option<Arc<AppHandle>>>,
    password: Mutex<Option<String>>,
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

pub fn init_vault_if_needed(
    app: &AppHandle,
    state: &VaultState,
    password: &str,
) -> Result<(), String> {
    let mut guard = state.inner.lock().unwrap();
    if guard.is_some() {
        return Ok(());
    }

    let v_path = vault_path(app);
    tracing::info!("Vault path: {:?}", v_path);

    let key = KeyDerivation::argon2(password, &salt_path(app));
    let key_provider = KeyProvider::try_from(zeroize::Zeroizing::new(key.clone()))
        .map_err(|e| format!("Failed to create KeyProvider: {:?}", e))?;

    let sh = Stronghold::new(v_path.clone(), key.clone()).map_err(|e| e.to_string())?;

    // Load existing snapshot if it exists
    if v_path.exists() {
        tracing::info!("Loading existing snapshot from {:?}", v_path);
        let snapshot_path = SnapshotPath::from_path(&v_path);
        if let Err(e) = sh.load_snapshot(&key_provider, &snapshot_path) {
            tracing::warn!("Failed to load existing snapshot: {:?}", e);
        } else {
            tracing::info!("Successfully loaded snapshot");
        }
    } else {
        tracing::info!("No existing snapshot found, creating new vault");
    }

    *guard = Some(sh);
    *state.app_handle.lock().unwrap() = Some(Arc::new(app.clone()));
    *state.password.lock().unwrap() = Some(password.to_string());
    Ok(())
}

fn commit_snapshot(state: &VaultState) -> Result<(), String> {
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

#[allow(dead_code)]
pub fn vault_init(
    app: AppHandle,
    state: State<VaultState>,
    password: String,
) -> Result<(), String> {
    let key = KeyDerivation::argon2(&password, &salt_path(&app)); // dérive une clé 32 bytes :contentReference[oaicite:2]{index=2}
    let sh = Stronghold::new(vault_path(&app), key).map_err(|e| e.to_string())?; // :contentReference[oaicite:3]{index=3}
    *state.inner.lock().unwrap() = Some(sh);
    Ok(())
}

fn with_sh<F, R>(state: &VaultState, f: F) -> Result<R, String>
where
    F: FnOnce(&Stronghold) -> Result<R, String>,
{
    let guard = state.inner.lock().unwrap();
    let sh = guard
        .as_ref()
        .ok_or("Vault not initialized (call vault_init)")?;
    f(sh)
}

#[tauri::command]
pub async fn get_account(
    state: State<'_, VaultState>,
    profile_name: &str,
) -> Result<Option<UserProfile>, String> {
    // Partie synchrone : lecture du profil depuis le vault
    let mut profile = with_sh(&state, |sh| {
        let client_path = format!("minecraft/{}", profile_name);
        let client = sh
            .load_client(client_path.as_bytes())
            .or_else(|_| sh.get_client(client_path.as_bytes()))
            .map_err(|e| e.to_string())?;
        let store = client.store();
        let username = match store.get(b"username").map_err(|e| e.to_string())? {
            Some(bytes) => String::from_utf8(bytes).map_err(|e| e.to_string())?,
            None => return Ok(None),
        };
        let uuid = match store.get(b"uuid").map_err(|e| e.to_string())? {
            Some(bytes) => String::from_utf8(bytes).map_err(|e| e.to_string())?,
            None => return Ok(None),
        };
        let access_token = match store.get(b"access_token").map_err(|e| e.to_string())? {
            Some(bytes) => String::from_utf8(bytes).map_err(|e| e.to_string())?,
            None => return Ok(None),
        };
        let refresh_token = match store.get(b"refresh_token").map_err(|e| e.to_string())? {
            Some(bytes) => String::from_utf8(bytes).map_err(|e| e.to_string())?,
            None => return Ok(None),
        };
        let expires_in = match store.get(b"expires_in").map_err(|e| e.to_string())? {
            Some(bytes) => String::from_utf8(bytes).map_err(|e| e.to_string())?,
            None => return Ok(None),
        };
        let emited_at = match store.get(b"emited_at").map_err(|e| e.to_string())? {
            Some(bytes) => String::from_utf8(bytes).map_err(|e| e.to_string())?,
            None => return Ok(None),
        };
        let provider_str = match store.get(b"provider").map_err(|e| e.to_string())? {
            Some(bytes) => String::from_utf8(bytes).map_err(|e| e.to_string())?,
            None => "unknown".to_string(),
        };
        let provider = match provider_str.as_str() {
            "microsoft" => AuthProvider::Microsoft {
                client_id: "7347d7b7-f14d-40c4-af19-f82204a7851e".to_string(),
            },
            "offline" => AuthProvider::Offline,
            _ => {
                tracing::warn!(
                    "Unknown provider '{}' for account '{}', defaulting to Offline",
                    provider_str,
                    profile_name
                );
                AuthProvider::Offline
            }
        };
        let mut profile = UserProfile {
            id: None,
            username,
            access_token: Some(access_token),
            refresh_token: Some(refresh_token),
            email: None,
            banned: false,
            email_verified: true,
            uuid,
            money: None,
            role: None,
            expires_in: expires_in.parse::<u64>().unwrap_or(0),
            emited_at: emited_at.parse::<DateTime<Utc>>().ok(),
            provider,
            refresh_impl: None,
        };
        profile.refresh_impl = match &profile.provider {
            AuthProvider::Microsoft { .. } => Some(Arc::new(MicrosoftRefresh)),
            AuthProvider::Offline => Some(Arc::new(OfflineRefresh)),
            AuthProvider::Azuriom { .. } => Some(Arc::new(lighty_auth::azuriom::AzuriomRefresh)),
            AuthProvider::Custom { .. } => None,
        };
        Ok(Some(profile))
    })?;
    // Si aucun profil trouvé
    let mut profile = match profile {
        Some(p) => p,
        None => return Ok(None),
    };
    // Partie asynchrone : refresh du token
    if let Some(refresh) = profile.refresh_impl.as_ref() {
        profile = refresh
            .refresh_access_token(&profile)
            .await
            .map_err(|e| format!("Failed to refresh profile: {}", e))?;
    } else {
        return Err("No refresh implementation (cannot refresh token)".to_string());
    }
    Ok(Some(profile))
}

#[tauri::command]
pub async fn add_account(
    state: State<'_, VaultState>,
    event_bus: State<'_, EventBus>,
    account_type: &str,
    profile_name: Option<String>,
) -> Result<(), String> {
    let profile: UserProfile;

    match account_type {
        "microsoft" => {
            profile = login_with_microsoft(event_bus).await?;
        }
        "offline" => {
            profile = login_offline(
                event_bus,
                profile_name.clone().ok_or("Profile name is required")?,
            )
            .await?;
        }
        _ => {
            return Err(format!("Unknown account type: {}", account_type));
        }
    }

    let path_name = profile_name.as_ref().unwrap_or_else(|| &profile.username);

    with_sh(&state, |sh| {
        let client_path = format!("minecraft/{}", path_name);
        let client = sh
            .get_client(client_path.as_bytes())
            .or_else(|_| sh.create_client(client_path.as_bytes()))
            .map_err(|e| e.to_string())?;

        let store = client.store();
        store
            .insert(
                "username".as_bytes().to_vec(),
                profile.username.as_bytes().to_vec(),
                None,
            )
            .map_err(|e| e.to_string())?;
        store
            .insert(
                "uuid".as_bytes().to_vec(),
                profile.uuid.as_bytes().to_vec(),
                None,
            )
            .map_err(|e| e.to_string())?;
        store
            .insert(
                "access_token".as_bytes().to_vec(),
                profile
                    .access_token
                    .as_deref()
                    .unwrap_or("")
                    .as_bytes()
                    .to_vec(),
                None,
            )
            .map_err(|e| e.to_string())?;
        store
            .insert(
                "refresh_token".as_bytes().to_vec(),
                profile
                    .refresh_token
                    .as_deref()
                    .unwrap_or("")
                    .as_bytes()
                    .to_vec(),
                None,
            )
            .map_err(|e| e.to_string())?;
        store
            .insert(
                "expires_in".as_bytes().to_vec(),
                profile.expires_in.to_string().as_bytes().to_vec(),
                None,
            )
            .map_err(|e| e.to_string())?;
        store
            .insert(
                "emited_at".as_bytes().to_vec(),
                profile
                    .emited_at
                    .map(|d| d.to_string())
                    .unwrap_or_default()
                    .as_bytes()
                    .to_vec(),
                None,
            )
            .map_err(|e| e.to_string())?;
        let provider_str = match &profile.provider {
            AuthProvider::Microsoft { .. } => "microsoft",
            AuthProvider::Offline => "offline",
            _ => "unknown",
        };
        store
            .insert(
                "provider".as_bytes().to_vec(),
                provider_str.as_bytes().to_vec(),
                None,
            )
            .map_err(|e| e.to_string())?;

        let metadata_client = sh
            .get_client(b"metadata/accounts")
            .or_else(|_| sh.create_client(b"metadata/accounts"))
            .map_err(|e| e.to_string())?;
        let metadata_store = metadata_client.store();
        let mut accounts: Vec<String> =
            match metadata_store.get(b"accounts").map_err(|e| e.to_string())? {
                Some(bytes) => serde_json::from_slice(&bytes).unwrap_or_default(),
                None => Vec::new(),
            };
        if !accounts.contains(&path_name) {
            accounts.push(path_name.clone());
        }
        tracing::info!("Saving accounts list: {:?}", accounts);
        metadata_store
            .insert(
                b"accounts".to_vec(),
                serde_json::to_vec(&accounts).map_err(|e| e.to_string())?,
                None,
            )
            .map_err(|e| e.to_string())?;

        // Commit changes to snapshot
        tracing::info!("Writing client to snapshot: {}", client_path);
        sh.write_client(client_path.as_bytes())
            .map_err(|e| e.to_string())?;
        sh.write_client(b"metadata/accounts")
            .map_err(|e| e.to_string())?;

        Ok(())
    })?;

    // Persist to disk
    tracing::info!("Committing snapshot to disk for account: {}", path_name);
    commit_snapshot(&state)?;
    tracing::info!("Account {} saved successfully", path_name);

    Ok(())
}

#[tauri::command]
pub fn del_account(state: State<'_, VaultState>, profile_name: &str) -> Result<(), String> {
    with_sh(&state, |sh| {
        let client_path = format!("minecraft/{}", profile_name);
        if let Ok(client) = sh.get_client(client_path.as_bytes()) {
            let store = client.store();
            let _ = store.delete(b"username");
            let _ = store.delete(b"uuid");
            let _ = store.delete(b"access_token");
        }

        // Mettre à jour la liste des comptes dans le metadata
        let metadata_client = sh
            .get_client(b"metadata/accounts")
            .or_else(|_| sh.create_client(b"metadata/accounts"))
            .map_err(|e| e.to_string())?;
        let metadata_store = metadata_client.store();
        let mut accounts: Vec<String> =
            match metadata_store.get(b"accounts").map_err(|e| e.to_string())? {
                Some(bytes) => serde_json::from_slice(&bytes).unwrap_or_default(),
                None => Vec::new(),
            };
        accounts.retain(|acc| acc != profile_name);
        metadata_store
            .insert(
                b"accounts".to_vec(),
                serde_json::to_vec(&accounts).map_err(|e| e.to_string())?,
                None,
            )
            .map_err(|e| e.to_string())?;

        // Commit uniquement le metadata/accounts (ne pas écrire le client supprimé)
        sh.write_client(b"metadata/accounts")
            .map_err(|e| e.to_string())?;
        Ok(())
    })?;
    commit_snapshot(&state)?;
    Ok(())
}

#[tauri::command]
pub async fn list_accounts(state: State<'_, VaultState>) -> Result<Vec<String>, String> {
    let guard = state.inner.lock().unwrap();
    let sh = match guard.as_ref() {
        Some(sh) => sh,
        None => {
            tracing::warn!("Vault not initialized when listing accounts");
            return Ok(Vec::new());
        }
    };

    // Try to load the metadata client from snapshot first
    let metadata_client = sh
        .load_client(b"metadata/accounts")
        .or_else(|_| {
            tracing::info!("Metadata client not in snapshot, using get_client");
            sh.get_client(b"metadata/accounts")
        })
        .or_else(|_| {
            tracing::info!("Creating new metadata client");
            sh.create_client(b"metadata/accounts")
        })
        .map_err(|e| e.to_string())?;

    let metadata_store = metadata_client.store();
    let accounts = match metadata_store.get(b"accounts").map_err(|e| e.to_string())? {
        Some(bytes) => {
            let accs: Vec<String> = serde_json::from_slice(&bytes).unwrap_or_default();
            tracing::info!("Found {} accounts: {:?}", accs.len(), accs);
            accs
        }
        None => {
            tracing::info!("No accounts found in metadata store");
            Vec::new()
        }
    };
    Ok(accounts)
}

#[tauri::command]
/// Login with Microsoft/Xbox Live
///
/// # Returns
/// A `UserProfile` containing the authenticated user's information, or an error message if authentication fails
pub async fn login_with_microsoft(event_bus: State<'_, EventBus>) -> Result<UserProfile, String> {
    let mut auth = MicrosoftAuth::new("7347d7b7-f14d-40c4-af19-f82204a7851e");

    auth.set_device_code_callback(|code, url| {
        println!("Please visit: {}", url);
        println!("And enter code: {}", code);
    });

    let profile = auth.authenticate(Some(&event_bus)).await.map_err(|e| {
        let msg = format!("Auth failed: {:?}", e);
        tracing::error!(%msg);
        msg
    })?;

    Ok(profile)
}

#[tauri::command]
/// Login with an offline account
///
/// # Arguments
/// - `username`: The username to login with (should not be empty)
/// # Returns
/// A `UserProfile` containing the authenticated user's information, or an error message if authentication fails
pub async fn login_offline(
    event_bus: State<'_, EventBus>,
    username: String,
) -> Result<UserProfile, String> {
    let mut auth = OfflineAuth::new(username);

    let profile = auth.authenticate(Some(&event_bus)).await.map_err(|e| {
        let msg = format!("Auth failed: {:?}", e);
        tracing::error!(%msg);
        msg
    })?;

    Ok(profile)
}
