use serde::{Deserialize, Serialize};
use serde_json;
use std::sync::Mutex;
use tauri::{command, AppHandle};
use tauri_plugin_store::StoreBuilder;

const SETTINGS_STORE: &str = "settings.json";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Settings {
    pub language: Option<String>,
    pub theme: Option<String>,
    // Ajoutez d'autres champs ici selon vos besoins
}

use std::collections::HashMap;
use std::sync::LazyLock;
static SETTINGS_CACHE: LazyLock<Mutex<HashMap<String, Settings>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Fonction utilitaire pour lire les settings d'un modpack
pub fn get_settings(app: &AppHandle, modpack_name: &str) -> Settings {
    let mut cache = SETTINGS_CACHE.lock().unwrap();
    if let Some(settings) = cache.get(modpack_name) {
        return settings.clone();
    }
    let store = StoreBuilder::new(app, std::path::Path::new(SETTINGS_STORE))
        .build()
        .expect("Erreur lors de la création du store");
    let value = store.get(modpack_name);
    let settings: Settings = match value {
        Some(val) => serde_json::from_value(val.clone()).unwrap_or_default(),
        None => Settings::default(),
    };
    cache.insert(modpack_name.to_string(), settings.clone());
    settings
}

/// Commande Tauri pour afficher les settings d'un modpack
#[command]
pub fn display_settings(app: AppHandle, modpack_name: String) -> Result<Settings, String> {
    Ok(get_settings(&app, &modpack_name))
}

/// Commande Tauri pour mettre à jour les settings d'un modpack
#[command]
pub fn update_settings(
    app: AppHandle,
    modpack_name: String,
    new_settings: Settings,
) -> Result<(), String> {
    let store = StoreBuilder::new(&app, std::path::Path::new(SETTINGS_STORE))
        .build()
        .map_err(|e| e.to_string())?;
    let value = serde_json::to_value(&new_settings).map_err(|e| e.to_string())?;
    store.set(&modpack_name, value);
    store.save().map_err(|e| e.to_string())?;
    let mut cache = SETTINGS_CACHE.lock().unwrap();
    cache.insert(modpack_name, new_settings);
    Ok(())
}
