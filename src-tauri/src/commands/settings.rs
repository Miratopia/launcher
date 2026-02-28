use lighty_launcher::JavaDistribution;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::sync::LazyLock;
use std::sync::Mutex;
use tauri::{command, AppHandle};
use tauri_plugin_store::StoreBuilder;

const SETTINGS_STORE: &str = "settings.json";

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
            java_distribution: Some(JavaDistribution::Temurin),
            min_memory: Some(1024),
            max_memory: Some(4096),
            full_screen: Some(false),
            window_width: Some(1280),
            window_height: Some(720),
        }
    }
}

/// Cache en mémoire des settings, indexé par nom de modpack.
///
/// Objectif : éviter de relire le store à chaque appel.
/// Le cache est mis à jour lors d’un `get_settings` (miss) et lors d’un `update_settings`.
static SETTINGS_CACHE: LazyLock<Mutex<HashMap<String, Settings>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Lit les settings d’un modpack depuis le cache ou, à défaut, depuis le store.
///
/// - Si le cache contient déjà l’entrée, on renvoie une copie.
/// - Sinon, on charge depuis le store (`settings.json`) à la clé `modpack_name`.
/// - Si la clé n’existe pas ou si la désérialisation échoue, on renvoie `Settings::default()`.
///
/// Le résultat est ensuite stocké dans le cache.
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

/// Commande Tauri : retourne les settings d’un modpack.
///
/// Cette commande renvoie toujours un `Settings` “utilisable” : si aucune valeur n’est stockée,
/// les valeurs par défaut sont renvoyées.
///
/// Note : étant donné `Settings::default()` et `unwrap_or_default()`, la plupart des champs
/// seront en pratique `Some(...)` dans le résultat.
#[command]
pub fn display_settings(app: AppHandle, modpack_name: String) -> Result<Settings, String> {
    Ok(get_settings(&app, &modpack_name))
}

/// Commande Tauri : écrit les settings d’un modpack dans le store et met à jour le cache.
///
/// ⚠️ Important : cette implémentation **n’effectue pas de merge**.
/// Elle sérialise `new_settings` tel quel et **remplace** la valeur stockée pour `modpack_name`.
///
/// Si tu veux une “mise à jour partielle” (où `None` signifie “ne pas toucher”),
/// il faut d’abord charger l’existant, puis ne remplacer que les champs `Some(...)`,
/// puis sauver le résultat fusionné.
#[command]
pub fn update_settings(
    app: AppHandle,
    modpack_name: String,
    new_settings: Settings,
) -> Result<Settings, String> {
    let store = StoreBuilder::new(&app, std::path::Path::new(SETTINGS_STORE))
        .build()
        .map_err(|e| e.to_string())?;
    let value = serde_json::to_value(&new_settings).map_err(|e| e.to_string())?;
    store.set(&modpack_name, value);
    store.save().map_err(|e| e.to_string())?;
    let mut cache = SETTINGS_CACHE.lock().unwrap();
    cache.insert(modpack_name.clone(), new_settings.clone());

    Ok(new_settings)
}
