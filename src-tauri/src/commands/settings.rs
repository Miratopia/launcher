use lighty_launcher::JavaDistribution;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use tauri::{command, AppHandle};
use tauri_plugin_store::StoreBuilder;

const SETTINGS_STORE: &str = "settings.json";

const MIN_MEMORY_DEFAULT: u32 = 1024;
const MAX_MEMORY_DEFAULT: u32 = 4096;
const FULL_SCREEN_DEFAULT: bool = false;
const WINDOW_WIDTH_DEFAULT: u32 = 1280;
const WINDOW_HEIGHT_DEFAULT: u32 = 720;
const JAVA_DISTRIBUTION_DEFAULT: JavaDistribution = JavaDistribution::Temurin;

const SETTINGS_KEY: &str = "settings";

/// Paramètres d’un modpack.
///
/// ## Pourquoi des `Option<T>` ?
/// Ce struct sert de **format de stockage** (et/ou de payload de mise à jour).
/// - `Some(v)` : la valeur est définie et doit être utilisée.
/// - `None` : la valeur est absente (non définie / inconnue).
///
/// ⚠️ Attention :
/// - `None` n’est pas “0” ni “false”.
/// - Et `None` n’est pas “null” *au sens logique* : en JSON ça peut être sérialisé en `null`,
///   mais en Rust ça veut surtout dire “absence de valeur”.
///
/// ## Valeurs par défaut
/// Si aucune donnée n’est présente dans le store (ou si elle est invalide),
/// on retombe sur `Settings::default()`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub java_distribution: Option<JavaDistribution>,
    pub min_memory: Option<u32>,
    pub max_memory: Option<u32>,
    pub full_screen: Option<bool>,
    pub window_width: Option<u32>,
    pub window_height: Option<u32>,
    /// Chemins (relatifs à la racine du modpack) des fichiers optionnels activés
    /// par l'utilisateur. `None` = jamais initialisé pour ce modpack.
    /// Sérialisé seulement si `Some` pour rester aligné avec le type TS
    /// (`optionalFiles?: string[]`) côté frontend.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional_files: Option<Vec<String>>,
    /// `true` lorsque le dialogue "premier lancement" pour les mods optionnels
    /// a déjà été présenté à l'utilisateur sur ce modpack.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional_files_prompted: Option<bool>,
}

/// Valeurs par défaut des paramètres.
///
/// Ces valeurs sont utilisées lorsque :
/// - le modpack n’a pas encore d’entrée dans le store ;
/// - ou que les données stockées ne peuvent pas être désérialisées.
///
/// Note : ici, les champs restent des `Option<T>`, mais le `default()` les remplit avec `Some(...)`.
impl Default for Settings {
    fn default() -> Self {
        Self {
            java_distribution: Some(JAVA_DISTRIBUTION_DEFAULT),
            min_memory: Some(MIN_MEMORY_DEFAULT),
            max_memory: Some(MAX_MEMORY_DEFAULT),
            full_screen: Some(FULL_SCREEN_DEFAULT),
            window_width: Some(WINDOW_WIDTH_DEFAULT),
            window_height: Some(WINDOW_HEIGHT_DEFAULT),
            optional_files: None,
            optional_files_prompted: None,
        }
    }
}

/// Lit les settings d’un modpack depuis le store (sous-clé modpacks).
///
/// - Forçage d’un `reload()` du store sous-jacent pour récupérer les modifications
///   éventuellement faites manuellement dans `settings.json` entre deux lectures
///   (le plugin `tauri-plugin-store` garde un cache process-wide qui survit aux
///   F5/recharges de webview).
/// - Charge la sous-clé `settings.{modpack_name}`.
/// - Si la clé n’existe pas ou si la désérialisation échoue, renvoie `Settings::default()`.
pub fn get_modpack_settings(app: &AppHandle, modpack_name: &str) -> Settings {
    let store = StoreBuilder::new(app, std::path::Path::new(SETTINGS_STORE))
        .build()
        .expect("Erreur lors de la création du store");

    if let Err(e) = store.reload() {
        tracing::debug!(
            "Failed to reload {} from disk (likely missing): {}",
            SETTINGS_STORE,
            e
        );
    }

    let modpacks_value = store.get(SETTINGS_KEY);
    match modpacks_value {
        Some(val) => {
            let map: HashMap<String, serde_json::Value> =
                serde_json::from_value(val.clone()).unwrap_or_default();
            match map.get(modpack_name) {
                Some(mp_val) => serde_json::from_value(mp_val.clone()).unwrap_or_default(),
                None => Settings::default(),
            }
        }
        None => Settings::default(),
    }
}

/// Commande Tauri : retourne les settings d’un modpack (clé modpacks).
///
/// Cette commande renvoie toujours un `Settings` “utilisable” : si aucune valeur n’est stockée,
/// les valeurs par défaut sont renvoyées.
#[command]
pub fn display_modpack_settings(app: AppHandle, modpack_name: String) -> Result<Settings, String> {
    Ok(get_modpack_settings(&app, &modpack_name))
}

/// Commande Tauri : écrit les settings d’un modpack dans la sous-clé modpacks.
///
/// ⚠️ Important : cette implémentation **n’effectue pas de merge**.
/// Elle sérialise `new_settings` tel quel et **remplace** la valeur stockée pour `modpack_name` dans la sous-clé modpacks.
#[command]
pub fn update_modpack_settings(
    app: AppHandle,
    modpack_name: String,
    new_settings: Settings,
) -> Result<Settings, String> {
    let store = StoreBuilder::new(&app, std::path::Path::new(SETTINGS_STORE))
        .build()
        .map_err(|e| e.to_string())?;

    if let Err(e) = store.reload() {
        tracing::debug!(
            "Failed to reload {} before update (likely missing): {}",
            SETTINGS_STORE,
            e
        );
    }

    let mut modpacks_map: HashMap<String, serde_json::Value> = match store.get(SETTINGS_KEY) {
        Some(val) => serde_json::from_value(val.clone()).unwrap_or_default(),
        None => HashMap::new(),
    };
    let value = serde_json::to_value(&new_settings).map_err(|e| e.to_string())?;
    modpacks_map.insert(modpack_name.clone(), value);
    store.set(
        SETTINGS_KEY,
        serde_json::to_value(&modpacks_map).map_err(|e| e.to_string())?,
    );
    store.save().map_err(|e| e.to_string())?;

    Ok(new_settings)
}

/// Reset all settings to the default values (efface `settings.json`).
#[command]
pub fn reset_all_settings(app: AppHandle) -> Result<(), String> {
    let store = StoreBuilder::new(&app, std::path::Path::new(SETTINGS_STORE))
        .build()
        .map_err(|e| e.to_string())?;

    store.clear();
    store.save().map_err(|e| e.to_string())?;

    tracing::info!("All settings have been reset");
    Ok(())
}
