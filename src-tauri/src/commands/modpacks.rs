use crate::commands::accounts::{display_account, display_active_account, get_active_account};
use crate::commands::settings::get_modpack_settings;
use crate::utils::vault::VaultState;
use lighty_launcher::_loaders::types::version_metadata::AssetsFile;
use lighty_launcher::prelude::InstanceControl;
use lighty_launcher::Loader;
use lighty_launcher::{loaders::{Asset, Mods}, prelude::*};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::State;

#[cfg(target_os = "windows")]
const FOLDER_OPENER: &str = "explorer";
#[cfg(target_os = "macos")]
const FOLDER_OPENER: &str = "open";
#[cfg(target_os = "linux")]
const FOLDER_OPENER: &str = "xdg-open";

static MC_INSTANCE: Lazy<Mutex<Option<VersionBuilder<'static, Loader>>>> =
    Lazy::new(|| Mutex::new(None));

fn extract_sha1_mismatch_path(error: &str) -> Option<String> {
    const MARKER: &str = "SHA1 mismatch for downloaded file: ";
    let start = error.find(MARKER)? + MARKER.len();
    let tail = &error[start..];
    let end = tail.find('"').unwrap_or(tail.len());
    let path = tail[..end].trim();

    if path.is_empty() {
        None
    } else {
        Some(path.to_string())
    }
}

fn normalize_loader_version(loader: &Loader, version: &str, minecraft_version: &str) -> String {
    // Certains anciens modpacks stockent NeoForge comme "<mc_version>-<loader_version>".
    // Le backend lighty reconstruit déjà la partie MC, ce qui crée un doublon
    // ("1.20.1-1.20.1-47.1.79"). On retire donc ce préfixe pour NeoForge.
    if matches!(loader, Loader::NeoForge) {
        let prefix = format!("{minecraft_version}-");
        if let Some(stripped) = version.strip_prefix(&prefix) {
            return stripped.to_string();
        }
    }

    version.to_string()
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftModpackInfo {
    pub version: String,
    pub recommended_memory: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoaderModpackInfo {
    #[serde(rename = "type")]
    pub loader_type: String,
    pub version: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FileModpackInfo {
    pub url: String,
    pub path: String,
    #[serde(default)]
    pub hash: Option<String>,
    #[serde(default)]
    pub size: Option<u64>,
    #[serde(default)]
    pub optional: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModpackInfo {
    pub id: String,
    pub name: String,
    #[serde(rename = "default")]
    pub default_modpack: bool,
    pub description: String,
    pub hidden: bool,

    #[serde(rename = "minecraft")]
    pub minecraft_info: MinecraftModpackInfo,

    #[serde(rename = "loaders")]
    pub modloader_info: Vec<LoaderModpackInfo>,

    #[serde(rename = "files")]
    pub files_info: Vec<FileModpackInfo>,

    #[serde(default)]
    pub whitelist: Option<Vec<String>>,

    #[serde(rename = "ignoredFiles", default)]
    pub ignored_files: Option<Vec<String>>,
}

#[tauri::command]
pub fn open_modpacks_folder() -> Result<(), String> {
    let launcher_dir = AppState::get_project_dirs();
    let data_path = launcher_dir.data_dir();

    // Créer le dossier s'il n'existe pas
    if !data_path.exists() {
        std::fs::create_dir_all(&data_path)
            .map_err(|e| format!("Failed to create data folder: {}", e))?;
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new(FOLDER_OPENER)
            .arg(&data_path)
            .spawn()
            .map_err(|e| format!("Failed to open data folder: {}", e))?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new(FOLDER_OPENER)
            .arg(&data_path)
            .spawn()
            .map_err(|e| format!("Failed to open data folder: {}", e))?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new(FOLDER_OPENER)
            .arg(&data_path)
            .spawn()
            .map_err(|e| format!("Failed to open data folder: {}", e))?;
    }
    Ok(())
}

/// Remove the contents of each modpack directory, preserving entries listed in
/// `config.ignored_files` from the remote `launcher.json`, plus each modpack's
/// `ignoredFiles` from its `modpack.json` (instance folders are named after `id`).
#[tauri::command]
pub async fn delete_all_modpacks() -> Result<(), String> {
    let url = "https://raw.githubusercontent.com/tacxtv/miratopia-launcher/refs/heads/config/launcher.json";
    let json: Value = reqwest::get(url)
        .await
        .map_err(|e| format!("Failed to download launcher.json: {}", e))?
        .json()
        .await
        .map_err(|e| format!("Failed to parse launcher.json: {}", e))?;

    let config = json.get("config").ok_or("No config found")?;
    let global_ignored: Vec<String> = config
        .get("ignored_files")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    let modpack_names: Vec<String> = config
        .get("modpacks")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|m| m.get("name").and_then(|n| n.as_str()).map(String::from))
                .collect()
        })
        .unwrap_or_default();

    let handles: Vec<_> = modpack_names
        .iter()
        .map(|name| {
            let name = name.clone();
            tokio::spawn(async move { fetch_modpack_info(&name).await })
        })
        .collect();

    let mut ignored_by_instance_id: HashMap<String, Vec<String>> = HashMap::new();
    for (handle, name) in handles.into_iter().zip(&modpack_names) {
        match handle.await {
            Ok(Ok(info)) => {
                if let Some(files) = info.ignored_files.filter(|f| !f.is_empty()) {
                    ignored_by_instance_id.insert(info.id, files);
                }
            }
            Ok(Err(e)) => tracing::warn!(
                modpack = %name,
                error = %e,
                "Could not fetch modpack.json for ignoredFiles merge"
            ),
            Err(e) => tracing::warn!(
                modpack = %name,
                error = %e,
                "Task failed while fetching modpack.json for ignoredFiles"
            ),
        }
    }

    let launcher_dir = AppState::get_project_dirs();
    let data_path = launcher_dir.data_dir();

    if !data_path.exists() {
        tracing::info!("Modpacks data directory does not exist, nothing to delete");
        return Ok(());
    }

    let entries = std::fs::read_dir(&data_path)
        .map_err(|e| format!("Failed to read data directory: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            let instance_id = entry.file_name().to_string_lossy().into_owned();
            let mut merged = global_ignored.clone();
            if let Some(extra) = ignored_by_instance_id.get(&instance_id) {
                merged.extend(extra.iter().cloned());
            }
            clean_modpack_dir(&path, &merged)?;
            tracing::info!("Cleaned modpack directory: {:?}", path);
        }
    }

    tracing::info!("All modpacks cleaned from {:?}", data_path);
    Ok(())
}

fn clean_modpack_dir(
    modpack_path: &std::path::Path,
    ignored_files: &[String],
) -> Result<(), String> {
    let entries = std::fs::read_dir(modpack_path)
        .map_err(|e| format!("Failed to read '{}': {}", modpack_path.display(), e))?;

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        if ignored_files.iter().any(|f| f == name_str.as_ref()) {
            tracing::info!("Preserving: {:?}", entry.path());
            continue;
        }

        let path = entry.path();
        if path.is_dir() {
            std::fs::remove_dir_all(&path)
                .map_err(|e| format!("Failed to delete '{}': {}", path.display(), e))?;
        } else {
            std::fs::remove_file(&path)
                .map_err(|e| format!("Failed to delete '{}': {}", path.display(), e))?;
        }
    }

    Ok(())
}

async fn fetch_modpack_info(name: &str) -> Result<ModpackInfo, String> {
    let url = format!(
        "https://raw.githubusercontent.com/tacxtv/miratopia-launcher/refs/heads/config/modpacks/{}/modpack.json",
        name,
    );
    let info = reqwest::get(&url)
        .await
        .map_err(|e| format!("Failed to download modpack info from {}: {}", url, e))?
        .json::<ModpackInfo>()
        .await
        .map_err(|e| format!("Failed to parse modpack JSON from {}: {}", url, e))?;
    Ok(info)
}

async fn fetch_modpack_additional_files(name: &str) -> Result<Vec<FileModpackInfo>, String> {
    let url = format!(
        "https://raw.githubusercontent.com/tacxtv/miratopia-launcher/refs/heads/config/modpacks/{}/files.json",
        name,
    );
    let files = reqwest::get(&url)
        .await
        .map_err(|e| format!("Failed to download files.json from {}: {}", url, e))?
        .json::<Vec<FileModpackInfo>>()
        .await
        .map_err(|e| format!("Failed to parse files.json from {}: {}", url, e))?;
    Ok(files)
}

#[tauri::command]
pub async fn list_modpacks(state: State<'_, VaultState>) -> Result<Vec<ModpackInfo>, String> {
    let profile_name = display_active_account(state.clone())
        .await
        .map_err(|e| format!("Failed to get active account: {}", e))?
        .ok_or_else(|| "No active profile".to_string())?
        .username;

    let url = "https://raw.githubusercontent.com/tacxtv/miratopia-launcher/refs/heads/config/launcher.json";
    let json: Value = reqwest::get(url)
        .await
        .map_err(|e| format!("Failed to download launcher.json: {}", e))?
        .json()
        .await
        .map_err(|e| format!("Failed to parse launcher.json: {}", e))?;

    let config = json.get("config").ok_or("No config found")?;
    let modpacks = config
        .get("modpacks")
        .and_then(|v| v.as_array())
        .ok_or("No modpacks array found")?;

    let profile = display_account(state, &profile_name)
        .await
        .map_err(|e| format!("Failed to get account: {}", e))?
        .ok_or_else(|| "Profile not found".to_string())?;
    let username = profile.username.to_lowercase();

    let mut allowed_names = Vec::new();
    for modpack in modpacks {
        let whitelisted = modpack
            .get("whitelisted")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let whitelist = modpack.get("whitelist").and_then(|v| v.as_array());
        let is_in_whitelist = whitelist.map_or(true, |arr| {
            arr.is_empty()
                || arr.iter().any(|u| {
                    u.as_str()
                        .map(|value| value.trim().to_lowercase() == username)
                        .unwrap_or(false)
                })
        });
        if whitelisted && is_in_whitelist || !whitelisted {
            if let Some(name) = modpack.get("name").and_then(|v| v.as_str()) {
                allowed_names.push(name.to_string());
            }
        }
    }

    let handles: Vec<_> = allowed_names
        .iter()
        .map(|name| {
            let name = name.clone();
            tokio::spawn(async move { fetch_modpack_info(&name).await })
        })
        .collect();

    let mut result = Vec::with_capacity(handles.len());
    for (handle, name) in handles.into_iter().zip(&allowed_names) {
        match handle.await {
            Ok(Ok(info)) => result.push(info),
            Ok(Err(e)) => tracing::warn!(modpack = %name, error = %e, "Skipping modpack"),
            Err(e) => tracing::warn!(modpack = %name, error = %e, "Skipping modpack (task failed)"),
        }
    }

    Ok(result)
}

#[tauri::command]
pub async fn start_modpack(
    app_handle: tauri::AppHandle,
    state: State<'_, VaultState>,
    event_bus: State<'_, EventBus>,
    modpack_name: String,
) -> Result<String, String> {
    let (instance_exit_tx, instance_exit_rx) = tokio::sync::oneshot::channel::<Option<i32>>();

    let mut receiver = event_bus.subscribe();
    let mut instance_exit_tx = Some(instance_exit_tx);

    tokio::spawn(async move {
        while let Ok(event) = receiver.next().await {
            match event {
                Event::ConsoleOutput(e) => {
                    let prefix = match e.stream {
                        ConsoleStream::Stdout => "[GAME]",
                        ConsoleStream::Stderr => "[ERR]",
                    };
                    println!("{} {}", prefix, e.line);
                }
                Event::InstanceExited(e) => {
                    println!("\n⚠ Instance exited with code: {:?}", e.exit_code);
                    if let Some(tx) = instance_exit_tx.take() {
                        let _ = tx.send(e.exit_code);
                    }
                }
                _ => {}
            }
        }
    });
    let launcher_dir = AppState::get_project_dirs();

    let settings = get_modpack_settings(&app_handle, &modpack_name);
    println!(
        "Loaded settings for modpack '{}': {:?}",
        modpack_name, settings
    );

    let profile = get_active_account(state.clone())
        .await
        .map_err(|e| format!("Failed to get active account: {}", e))?
        .ok_or_else(|| "No active profile".to_string())?;

    println!("profile: {} (uuid: {})", profile.username, profile.uuid);
    println!("Authentication completed.");

    let modpack_url = format!(
        "https://raw.githubusercontent.com/tacxtv/miratopia-launcher/refs/heads/config/modpacks/{}/modpack.json",
        modpack_name,
    );
    let modpack = reqwest::get(&modpack_url)
        .await
        .map_err(|e| {
            let msg = format!("Failed to download modpack from {}: {}", modpack_url, e);
            tracing::error!(%msg);
            msg
        })?
        .json::<ModpackInfo>()
        .await
        .map_err(|e| {
            let msg = format!("Failed to parse modpack JSON from {}: {}", modpack_url, e);
            tracing::error!(%msg);
            msg
        })?;

    let loader_type = match modpack
        .modloader_info
        .get(0)
        .map(|m| m.loader_type.as_str())
    {
        Some("fabric") => Loader::Fabric,
        Some("forge") => Loader::Forge,
        Some("neoforge") => Loader::NeoForge,
        Some("quilt") => Loader::Quilt,
        _ => {
            return Err(format!(
                "Unknown loader type in modpack: {:?}",
                modpack
                    .modloader_info
                    .get(0)
                    .map(|m| m.loader_type.as_str())
            ))
        }
    };

    println!("Launching game with modpack: {:?}", modpack);

    let normalized_loader_version = normalize_loader_version(
        &loader_type,
        modpack
            .modloader_info
            .get(0)
            .map(|m| m.version.as_str())
            .expect("Modloader version is required"),
        modpack.minecraft_info.version.as_str(),
    );

    let mut instance = VersionBuilder::new(
        &modpack.id,
        loader_type,
        normalized_loader_version.as_str(),
        modpack.minecraft_info.version.as_str(),
        launcher_dir,
    );

    let mut mods = Vec::new();
    for file in &modpack.files_info {
        if file.path.contains("mods/") {
            if file.hash.is_none() {
                tracing::warn!(
                    "Missing hash for mod file '{}'; continuing without sha1 verification",
                    file.path
                );
            }
            mods.push(Mods {
                name: file.path.clone(),
                path: Some(file.path.clone().replace("mods/", "")),
                url: Some(file.url.clone()),
                sha1: file.hash.clone(),
                size: file.size,
            });
        }
    }

    instance = instance.with_mods(mods);

    let additional_files = fetch_modpack_additional_files(&modpack_name).await?;
    let mut assets = HashMap::new();
    for file in additional_files {
        match (file.hash, file.size) {
            (Some(hash), Some(size)) => {
                assets.insert(
                    file.path,
                    Asset {
                        hash,
                        size,
                        url: Some(file.url),
                    },
                );
            }
            _ => tracing::warn!(
                "Skipping additional file '{}' because hash or size is missing",
                file.path
            ),
        }
    }
    if !assets.is_empty() {
        instance = instance.with_assets(AssetsFile { objects: assets });
    }


    // Stocke l'instance dans la variable globale
    // {
    //     let mut guard = MC_INSTANCE.lock().unwrap();
    //     *guard = Some(instance.clone());
    // }

    // println!(
    //     "profile: {} (uuid: {}, token: {:?}, refresh: {:?})",
    //     profile.username,
    //     profile.uuid,
    //     profile.access_token,
    //     profile.refresh_token,
    // );

    let java_distribution = settings
        .java_distribution
        .unwrap_or(JavaDistribution::Temurin);
    let max_memory = settings.max_memory.unwrap_or(4096);
    let min_memory = settings.min_memory.unwrap_or(2048);

    let launch_result = instance
        .launch(
            &profile,
            java_distribution.clone(),
        )
        .with_event_bus(&event_bus.inner().clone())
        .with_jvm_options()
        .set("Xmx", max_memory.to_string() + "M")
        .set("Xms", min_memory.to_string() + "M")
        .done()
        .run()
        .await;

    match launch_result {
        Ok(_) => {}
        Err(first_error) => {
            let first_debug = format!("{:?}", first_error);
            if let Some(corrupted_path) = extract_sha1_mismatch_path(&first_debug) {
                tracing::warn!(
                    path = %corrupted_path,
                    "Detected SHA1 mismatch, deleting corrupted file and retrying launch once"
                );
                if let Err(remove_error) = std::fs::remove_file(&corrupted_path) {
                    tracing::warn!(
                        path = %corrupted_path,
                        error = %remove_error,
                        "Unable to remove corrupted file before retry"
                    );
                }

                instance
                    .launch(&profile, java_distribution)
                    .with_event_bus(&event_bus.inner().clone())
                    .with_jvm_options()
                    .set("Xmx", max_memory.to_string() + "M")
                    .set("Xms", min_memory.to_string() + "M")
                    .done()
                    .run()
                    .await
                    .map_err(|retry_error| {
                        let msg = format!("Launch failed after SHA1 retry: {:?}", retry_error);
                        tracing::error!(%msg);
                        msg
                    })?;
            } else {
                let msg = format!("Launch failed: {:?}", first_error);
                tracing::error!(%msg);
                return Err(msg);
            }
        }
    }

    let _ = instance_exit_rx.await;
    Ok(format!("Game {} launched successfully", modpack_name))
}

#[tauri::command]
pub async fn stop_modpack(
    _event_bus: State<'_, EventBus>,
    _instance_id: String,
) -> Result<String, String> {
    // On extrait l'instance pour ne pas garder le lock pendant l'await
    let instance_opt = {
        let mut guard = MC_INSTANCE.lock().unwrap();
        guard.take()
    };
    if let Some(instance) = instance_opt {
        if let Some(pid) = instance.get_pid() {
            println!("Running with PID: {}", pid);
            instance
                .close_instance(pid)
                .await
                .map_err(|e| format!("Erreur fermeture: {:?}", e))?;
        }
        Ok("Instance arrêtée".to_string())
    } else {
        Err("Instance non trouvée".to_string())
    }
}

// pub fn get_instance(_instance_id: String) -> Result<bool, String> {
//     let guard = MC_INSTANCE.lock().unwrap();
//     Ok(guard.is_some())
// }
