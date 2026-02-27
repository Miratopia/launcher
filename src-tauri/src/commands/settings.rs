use serde::{Deserialize, Serialize};
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

static SETTINGS_CACHE: Mutex<Option<Settings>> = Mutex::new(None);

/// Fonction utilitaire pour lire les settings (utilisable dans game)
pub fn get_settings(app: &AppHandle) -> Settings {
    // On utilise le cache pour éviter de relire à chaque fois
    let mut cache = SETTINGS_CACHE.lock().unwrap();
    if let Some(settings) = cache.clone() {
        return settings;
    }
    let store = StoreBuilder::new(app, SETTINGS_STORE.into()).build();
    let settings: Settings = store.load().unwrap_or_default();
    *cache = Some(settings.clone());
    settings
}

/// Commande Tauri pour afficher les settings
#[command]
pub fn display_settings(app: AppHandle) -> Result<Settings, String> {
    Ok(get_settings(&app))
}

/// Commande Tauri pour mettre à jour les settings
#[command]
pub fn update_settings(app: AppHandle, new_settings: Settings) -> Result<(), String> {
    let store = StoreBuilder::new(&app, SETTINGS_STORE.into()).build();
    store.save(&new_settings).map_err(|e| e.to_string())?;
    let mut cache = SETTINGS_CACHE.lock().unwrap();
    *cache = Some(new_settings);
    Ok(())
}
