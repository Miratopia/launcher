use crate::commands::accounts::get_account;
use crate::commands::settings::get_modpack_settings;
use crate::utils::vault::VaultState;
use lighty_launcher::prelude::InstanceControl;
use lighty_launcher::Loader;
use lighty_launcher::{loaders::Mods, prelude::*};
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::sync::Mutex;
use tauri::State;

static MC_INSTANCE: Lazy<Mutex<Option<VersionBuilder<'static, Loader>>>> =
    Lazy::new(|| Mutex::new(None));

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct MinecraftModpackInfo {
    version: String,
    #[serde(rename = "recommendedMemory")]
    recommended_memory: u32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct LoaderModpackInfo {
    #[serde(rename = "type")]
    loader_type: String,
    version: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct FileModpackInfo {
    url: String,
    path: String,
    hash: String,
    size: u64,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct ModpackInfo {
    id: String,
    name: String,
    #[serde(rename = "default")]
    default_modpack: bool,
    description: String,
    hidden: bool,

    #[serde(rename = "minecraft")]
    minecraft_info: MinecraftModpackInfo,

    #[serde(rename = "loaders")]
    modloader_info: Vec<LoaderModpackInfo>,

    #[serde(rename = "files")]
    files_info: Vec<FileModpackInfo>,

    whitelist: Vec<String>,

    #[serde(rename = "ignoredFiles")]
    ignored_files: Vec<String>,
}

#[tauri::command]
pub async fn start_modpack(
    app_handle: tauri::AppHandle,
    state: State<'_, VaultState>,
    event_bus: State<'_, EventBus>,
    modpack_name: String,
    profile_name: String,
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

    let profile = get_account(state, &profile_name)
        .await
        .map_err(|e| format!("Failed to get account: {}", e))?
        .ok_or_else(|| "Profile not found".to_string())?;

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

    let mut instance = VersionBuilder::new(
        &modpack.id,
        loader_type,
        modpack
            .modloader_info
            .get(0)
            .map(|m| m.version.as_str())
            .expect("Modloader version is required"),
        modpack.minecraft_info.version.as_str(),
        launcher_dir,
    );

    let mut mods = Vec::new();
    for file in &modpack.files_info {
        if file.path.contains("mods/") {
            mods.push(Mods {
                name: file.path.clone(),
                path: Some(file.path.clone().replace("mods/", "")),
                url: Some(file.url.clone()),
                sha1: Some(file.hash.clone()),
                size: Some(file.size),
            });
        }
    }

    instance = instance.with_mods(mods);

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

    instance
        .launch(
            &profile,
            settings
                .java_distribution
                .unwrap_or(JavaDistribution::Temurin),
        )
        .with_event_bus(&event_bus.inner().clone())
        .with_jvm_options()
        .set("Xmx", settings.max_memory.unwrap_or(4096).to_string() + "M")
        .set("Xms", settings.min_memory.unwrap_or(2048).to_string() + "M")
        .done()
        .run()
        .await
        .map_err(|e| {
            let msg = format!("Launch failed: {:?}", e);
            tracing::error!(%msg);
            msg
        })?;

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
