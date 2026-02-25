use lighty_launcher::{loaders::Mods, prelude::*};
use serde::Deserialize;
use tauri::State;

use crate::commands::accounts::{get_account, VaultState};

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
pub async fn launch_game(
    state: State<'_, VaultState>,
    event_bus: State<'_, EventBus>,
    modpack_name: String,
    profile_name: String,
    java_distribution: String,
) -> Result<String, String> {
    // let event_bus = EventBus::new(1000);
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

    let java_dist = match java_distribution.as_str() {
        "temurin" => JavaDistribution::Temurin,
        "graalvm" => JavaDistribution::GraalVM,
        "zulu" => JavaDistribution::Zulu,
        "liberica" => JavaDistribution::Liberica,
        _ => return Err(format!("Unknown java distribution: {}", java_distribution)),
    };

    let profile = get_account(state, &profile_name)
        .await
        .map_err(|e| format!("Failed to get account: {}", e))?
        .ok_or_else(|| "Profile not found".to_string())?;

    println!("profile: {:?}", profile);
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

    instance
        .launch(&profile, java_dist)
        .with_event_bus(&event_bus.inner().clone())
        // .with_arguments({
        //     let mut args = HashMap::new();
        //     args.insert("demo".to_string(), "true".to_string());
        //     args
        // })
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
