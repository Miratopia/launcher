use crate::utils::vault::{commit_snapshot, with_sh, VaultState};
use chrono::{DateTime, Utc};
use lighty_auth::{microsoft::MicrosoftRefresh, offline::OfflineRefresh, AuthProvider};
use lighty_launcher::auth::{MicrosoftAuth, OfflineAuth};
use lighty_launcher::event::EventBus;
use lighty_launcher::{Authenticator, UserProfile};
use open;
use serde::Serialize;
use std::sync::Arc;
use std::time::Duration;
use tauri::State;
use tauri::{AppHandle, Emitter};

#[derive(Serialize)]
pub struct UserProfilePartial {
    pub username: String,
    pub uuid: String,
}

#[tauri::command]
pub async fn display_active_account(
    state: State<'_, VaultState>,
) -> Result<Option<UserProfilePartial>, String> {
    let active_profile = with_sh(
        &state,
        |sh: &tauri_plugin_stronghold::stronghold::Stronghold| {
            let metadata_client = sh
                .get_client(b"metadata/active_account")
                .or_else(|_| sh.create_client(b"metadata/active_account"))
                .map_err(|e| e.to_string())?;
            let metadata_store = metadata_client.store();
            let name = metadata_store
                .get(b"active_account")
                .map_err(|e| e.to_string())?;
            match name {
                Some(bytes) => String::from_utf8(bytes).map_err(|e| e.to_string()),
                None => Ok(String::new()),
            }
        },
    )?;
    if active_profile.is_empty() {
        // Si aucun compte actif, on prend le premier de la liste
        let accounts = list_accounts(state.clone()).await?;
        if let Some(first) = accounts.first() {
            return display_account(state, &first).await;
        } else {
            return Ok(None);
        }
    }
    display_account(state, &active_profile).await
}

#[tauri::command]
pub fn switch_active_account(
    state: State<'_, VaultState>,
    profile_name: &str,
) -> Result<(), String> {
    with_sh(
        &state,
        |sh: &tauri_plugin_stronghold::stronghold::Stronghold| {
            // On stocke le nom du compte actif dans le client metadata/active_account
            let metadata_client = sh
                .get_client(b"metadata/active_account")
                .or_else(|_| sh.create_client(b"metadata/active_account"))
                .map_err(|e| e.to_string())?;
            let metadata_store = metadata_client.store();
            metadata_store
                .insert(
                    b"active_account".to_vec(),
                    profile_name.as_bytes().to_vec(),
                    None,
                )
                .map_err(|e| e.to_string())?;
            // Commit le client
            sh.write_client(b"metadata/active_account")
                .map_err(|e| e.to_string())?;
            Ok(())
        },
    )?;
    // Persist to disk
    commit_snapshot(&state)?;
    Ok(())
}

#[tauri::command]
pub async fn display_account(
    state: State<'_, VaultState>,
    profile_name: &str,
) -> Result<Option<UserProfilePartial>, String> {
    let profile = with_sh(
        &state,
        |sh: &tauri_plugin_stronghold::stronghold::Stronghold| {
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
            let profile = UserProfilePartial { username, uuid };
            Ok(Some(profile))
        },
    )?;
    let profile = match profile {
        Some(p) => p,
        None => return Ok(None),
    };
    Ok(Some(profile))
}

#[tauri::command]
pub async fn get_active_account(
    state: State<'_, VaultState>,
) -> Result<Option<UserProfile>, String> {
    // Lire le nom du compte actif depuis le stronghold
    let active_profile = with_sh(
        &state,
        |sh: &tauri_plugin_stronghold::stronghold::Stronghold| {
            let metadata_client = sh
                .get_client(b"metadata/active_account")
                .or_else(|_| sh.create_client(b"metadata/active_account"))
                .map_err(|e| e.to_string())?;
            let metadata_store = metadata_client.store();
            let name = metadata_store
                .get(b"active_account")
                .map_err(|e| e.to_string())?;
            match &name {
                Some(bytes) => {
                    tracing::info!("Valeur brute active_account lue: {:?}", bytes);
                    String::from_utf8(bytes.clone()).map_err(|e| e.to_string())
                }
                None => {
                    tracing::info!("Aucune valeur active_account trouvée dans le stronghold");
                    Ok(String::new())
                }
            }
        },
    )?;
    if active_profile.is_empty() {
        // Si aucun compte actif, on prend le premier de la liste
        let accounts = list_accounts(state.clone()).await?;
        if let Some(first) = accounts.first() {
            return get_account(state, first).await;
        } else {
            return Ok(None);
        }
    }
    get_account(state, &active_profile).await
}

pub async fn get_account(
    state: State<'_, VaultState>,
    profile_name: &str,
) -> Result<Option<UserProfile>, String> {
    // Partie synchrone : lecture du profil depuis le vault
    let profile = with_sh(
        &state,
        |sh: &tauri_plugin_stronghold::stronghold::Stronghold| {
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
                AuthProvider::Azuriom { .. } => {
                    Some(Arc::new(lighty_auth::azuriom::AzuriomRefresh))
                }
                AuthProvider::Custom { .. } => None,
            };
            Ok(Some(profile))
        },
    )?;
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
    app_handle: AppHandle,
    state: State<'_, VaultState>,
    event_bus: State<'_, EventBus>,
    account_type: &str,
    profile_name: Option<String>,
) -> Result<(), String> {
    let profile: UserProfile;

    match account_type {
        "microsoft" => {
            profile = login_with_microsoft_app(app_handle, event_bus).await?;
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

    with_sh(
        &state,
        |sh: &tauri_plugin_stronghold::stronghold::Stronghold| {
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
        },
    )?;

    // Persist to disk
    tracing::info!("Committing snapshot to disk for account: {}", path_name);
    commit_snapshot(&state)?;
    tracing::info!("Account {} saved successfully", path_name);

    Ok(())
}

#[tauri::command]
pub fn del_account(state: State<'_, VaultState>, profile_name: &str) -> Result<(), String> {
    with_sh(
        &state,
        |sh: &tauri_plugin_stronghold::stronghold::Stronghold| {
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
        },
    )?;
    commit_snapshot(&state)?;
    Ok(())
}

#[tauri::command]
pub async fn list_accounts(state: State<'_, VaultState>) -> Result<Vec<String>, String> {
    let guard = state.inner.lock().unwrap();
    let sh: &tauri_plugin_stronghold::stronghold::Stronghold = match guard.as_ref() {
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

/// Login with an offline account
///
/// # Arguments
/// - `username`: The username to login with (should not be empty)
/// # Returns
/// A `UserProfile` containing the authenticated user's information, or an error message if authentication fails
async fn login_offline(
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

/// Login with Microsoft account using device code flow
///
/// # Arguments
/// - `app_handle`: The Tauri app handle for emitting events
/// - `event_bus`: The event bus for emitting auth events
/// # Returns
/// A `UserProfile` containing the authenticated user's information, or an error message if authentication fails
async fn login_with_microsoft_app(
    app_handle: AppHandle,
    event_bus: State<'_, EventBus>,
) -> Result<UserProfile, String> {
    let mut auth = MicrosoftAuth::new("7347d7b7-f14d-40c4-af19-f82204a7851e");
    auth.set_poll_interval(Duration::from_secs(5));
    auth.set_timeout(Duration::from_secs(60));
    auth.set_device_code_callback(move |code, url| {
        if let Err(e) = open::that(url) {
            println!("Erreur lors de l'ouverture du navigateur: {:?}", e);
        }
        let _ = app_handle.emit(
            "lighty://auth-microsoft-code",
            serde_json::json!({
                "code": code,
                "url": url,
            }),
        );
        println!("And enter code: {}", code);
    });
    let profile = auth.authenticate(Some(&event_bus)).await.map_err(|e| {
        let msg = format!("Auth failed: {:?}", e);
        tracing::error!(%msg);
        msg
    })?;
    Ok(profile)
}
