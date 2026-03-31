//! Mot de passe du Stronghold : généré et conservé hors du code source.
//! Windows : registre HKCU (Software\\Miratopia\\Launcher).
//! Autres OS : fichier dans app_local_data (CI / macOS / Linux).

use rand::Rng;
use std::path::Path;
use tauri::AppHandle;
use tauri::Manager;

/// Ancien secret inline ; utilisé une seule fois si un vault existe déjà sans entrée registre/fichier.
const LEGACY_INLINE_PASSWORD: &str = "dev-vault-password";

/// Aligné sur `utils::vault` (fichier snapshot Stronghold).
const VAULT_SNAPSHOT_NAME: &str = "vault.hold";

#[cfg(windows)]
mod windows_store {
    use super::*;
    use winreg::enums::*;
    use winreg::RegKey;

    const SUBKEY: &str = r"Software\Miratopia\Launcher";
    const VALUE_NAME: &str = "VaultPassword";

    pub fn get_or_create(vault_path: &Path) -> Result<String, String> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let (key, _) = hkcu
            .create_subkey(SUBKEY)
            .map_err(|e| format!("registre vault: création sous-clé: {e}"))?;

        match key.get_value::<String, _>(VALUE_NAME) {
            Ok(s) if !s.is_empty() => Ok(s),
            _ => {
                let pwd = if vault_path.exists() {
                    LEGACY_INLINE_PASSWORD.to_string()
                } else {
                    generate_random_password()
                };
                key.set_value(VALUE_NAME, &pwd)
                    .map_err(|e| format!("registre vault: écriture mot de passe: {e}"))?;
                Ok(pwd)
            }
        }
    }
}

#[cfg(not(windows))]
mod file_store {
    use super::*;
    use std::fs;
    use std::io::{Read, Write};

    const VAULT_PASSWORD_FILENAME: &str = ".vault_password";

    pub fn get_or_create(app: &AppHandle, vault_path: &Path) -> Result<String, String> {
        let path = app
            .path()
            .app_local_data_dir()
            .map_err(|e| format!("app_local_data_dir: {e}"))?
            .join(VAULT_PASSWORD_FILENAME);

        if path.exists() {
            let mut s = String::new();
            fs::File::open(&path)
                .and_then(|mut f| f.read_to_string(&mut s))
                .map_err(|e| format!("lecture mot de passe vault: {e}"))?;
            let s = s.trim().to_string();
            if s.is_empty() {
                return Err("mot de passe vault fichier vide".to_string());
            }
            return Ok(s);
        }

        let pwd = if vault_path.exists() {
            LEGACY_INLINE_PASSWORD.to_string()
        } else {
            generate_random_password()
        };

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("création répertoire mot de passe vault: {e}"))?;
        }
        let mut f =
            fs::File::create(&path).map_err(|e| format!("création fichier mot de passe vault: {e}"))?;
        f.write_all(pwd.as_bytes())
            .map_err(|e| format!("écriture mot de passe vault: {e}"))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = fs::set_permissions(&path, fs::Permissions::from_mode(0o600));
        }
        Ok(pwd)
    }
}

fn generate_random_password() -> String {
    const CHARSET: &[u8] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let mut rng = rand::thread_rng();
    (0..64)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/// Retourne le mot de passe Stronghold (registre Windows ou fichier selon la plateforme).
pub fn get_or_create_vault_password(app: &AppHandle) -> Result<String, String> {
    let vault_path = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("app_data_dir: {e}"))?
        .join(VAULT_SNAPSHOT_NAME);

    #[cfg(windows)]
    {
        windows_store::get_or_create(&vault_path)
    }
    #[cfg(not(windows))]
    {
        file_store::get_or_create(app, &vault_path)
    }
}
