use serde::{Deserialize, Serialize};

/// Phase de téléchargement/installation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DownloadPhase {
    Idle,
    Java,
    Loader,
    Game,
    Assets,
    Libraries,
    Extracting,
}

/// Événement de progression de téléchargement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadProgressPayload {
    pub phase: DownloadPhase,
    pub current_bytes: u64,
    pub total_bytes: u64,
    pub percentage: u8,
    pub message: String,
    pub instance_name: String,
}

/// Statut de lancement
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LaunchStatus {
    Initializing,
    Downloading,
    Installing,
    Installed,
    Running,
    Launched,
    Exited,
    Failed,
}

/// Événement de changement de statut
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchStatusPayload {
    pub status: LaunchStatus,
    pub phase: String,
    pub instance_name: String,
    pub pid: Option<u32>,
}

/// Ligne de console
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsoleLinePayload {
    pub instance_name: String,
    pub pid: u32,
    pub stream: String, // "stdout" | "stderr"
    pub line: String,
    pub timestamp: u64,
}

/// Erreur
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorPayload {
    pub category: String, // "auth", "java", "launch", "loader", "core"
    pub message: String,
    pub details: Option<String>,
    pub timestamp: u64,
}
