use lighty_launcher::event::{Event, EventBus};
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::Mutex;
use tokio::time::{Duration, Instant};
use tracing::info;

use crate::types::*;

/// State partagé pour tracking du total_bytes et instance
#[derive(Debug, Clone, Default)]
struct ProgressState {
    java_total: u64,
    java_downloaded: u64,
    install_total: u64,
    install_downloaded: u64,
    current_instance: String,
}

/// Lanceur d'écoute des événements LightyLauncher
/// Convertit les événements EventBus en événements Tauri
pub fn spawn_event_listener(app: AppHandle, event_bus: EventBus) {
    tauri::async_runtime::spawn(async move {
        let mut receiver = event_bus.subscribe();

        // État pour throttling
        let last_progress_emit = Arc::new(Mutex::new(Instant::now()));
        let throttle_duration = Duration::from_millis(100); // 10 updates/sec max

        // État partagé pour total_bytes
        let progress_state = Arc::new(Mutex::new(ProgressState::default()));

        // Buffer pour console (éviter spam)
        let console_buffer: Arc<Mutex<Vec<ConsoleLinePayload>>> = Arc::new(Mutex::new(Vec::new()));
        let buffer_flush_interval = Duration::from_millis(250);

        // Task pour flush périodique du buffer console
        let console_buffer_clone = console_buffer.clone();
        let app_clone = app.clone();
        tauri::async_runtime::spawn(async move {
            let mut interval = tokio::time::interval(buffer_flush_interval);
            loop {
                interval.tick().await;
                let mut buffer: tokio::sync::MutexGuard<'_, Vec<ConsoleLinePayload>> =
                    console_buffer_clone.lock().await;
                if !buffer.is_empty() {
                    let lines = buffer.drain(..).collect::<Vec<_>>();
                    let _ = app_clone.emit("lighty://console-output", lines);
                }
            }
        });

        loop {
            match receiver.next().await {
                Ok(event) => {
                    handle_event(
                        &app,
                        event,
                        &last_progress_emit,
                        &console_buffer,
                        &progress_state,
                        throttle_duration,
                    )
                    .await;
                }
                Err(e) => {
                    eprintln!("Event receiver error: {:?}", e);
                    break;
                }
            }
        }
    });
}

async fn handle_event(
    app: &AppHandle,
    event: Event,
    last_emit: &Arc<Mutex<Instant>>,
    console_buffer: &Arc<Mutex<Vec<ConsoleLinePayload>>>,
    progress_state: &Arc<Mutex<ProgressState>>,
    throttle: Duration,
) {
    match event {
        // === JAVA EVENTS ===
        Event::Java(java_event) => {
            handle_java_event(app, java_event, last_emit, progress_state, throttle).await
        }

        // === LAUNCH EVENTS ===
        Event::Launch(launch_event) => {
            handle_launch_event(app, launch_event, last_emit, progress_state, throttle).await
        }

        // === LOADER EVENTS ===
        Event::Loader(loader_event) => {
            use lighty_launcher::event::LoaderEvent;

            match loader_event {
                LoaderEvent::FetchingData {
                    loader,
                    minecraft_version,
                    loader_version,
                } => {
                    let payload = LaunchStatusPayload {
                        status: LaunchStatus::Initializing,
                        phase: format!("Fetching {} {}", loader, loader_version),
                        instance_name: minecraft_version,
                        pid: 0,
                    };
                    let _ = app.emit("lighty://launch-status", payload);
                }
                LoaderEvent::DataFetched { loader, .. } => {
                    let payload = LaunchStatusPayload {
                        status: LaunchStatus::Initializing,
                        phase: format!("{} metadata loaded", loader),
                        instance_name: String::new(),
                        pid: 0,
                    };
                    let _ = app.emit("lighty://launch-status", payload);
                }
                _ => {}
            }
        }

        // === CORE EVENTS ===
        Event::Core(core_event) => handle_core_event(app, core_event).await,

        // === INSTANCE EVENTS ===
        Event::InstanceLaunched(evt) => {
            let payload = LaunchStatusPayload {
                status: LaunchStatus::Running,
                phase: "Game running".to_string(),
                instance_name: evt.instance_name,
                pid: evt.pid,
            };
            let _ = app.emit("lighty://launch-status", payload);
        }

        Event::InstanceWindowAppeared(evt) => {
            let payload = LaunchStatusPayload {
                status: LaunchStatus::Launched,
                phase: "Game window opened".to_string(),
                instance_name: evt.instance_name,
                pid: evt.pid,
            };
            let _ = app.emit("lighty://launch-status", payload);

            // Cacher la fenêtre dans le system tray quand le jeu apparaît
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.hide();
            }
        }

        Event::InstanceExited(evt) => {
            let payload = LaunchStatusPayload {
                status: LaunchStatus::Exited,
                phase: format!("Game exited (code: {:?})", evt.exit_code),
                instance_name: evt.instance_name,
                pid: 0,
            };
            let _ = app.emit("lighty://launch-status", payload);

            // Afficher la fenêtre du launcher quand le jeu se ferme
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }

        // === CONSOLE OUTPUT ===
        Event::ConsoleOutput(evt) => {
            let line = ConsoleLinePayload {
                instance_name: evt.instance_name,
                pid: evt.pid,
                stream: match evt.stream {
                    lighty_launcher::event::ConsoleStream::Stdout => "stdout".to_string(),
                    lighty_launcher::event::ConsoleStream::Stderr => "stderr".to_string(),
                },
                line: evt.line,
                timestamp: evt
                    .timestamp
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            };

            console_buffer.lock().await.push(line);
        }

        Event::InstanceDeleted(evt) => {
            let payload = LaunchStatusPayload {
                status: LaunchStatus::Exited,
                phase: "Instance deleted".to_string(),
                instance_name: evt.instance_name,
                pid: 0,
            };
            let _ = app.emit("lighty://launch-status", payload);
        }

        // === AUTH EVENTS ===
        Event::Auth(auth_event) => {
            use lighty_launcher::event::AuthEvent;

            match auth_event {
                AuthEvent::AuthenticationStarted { provider } => {
                    let payload = LaunchStatusPayload {
                        status: LaunchStatus::Initializing,
                        phase: format!("Authenticating with {}", provider),
                        instance_name: String::new(),
                        pid: 0,
                    };
                    let _ = app.emit("lighty://launch-status", payload);
                }
                AuthEvent::AuthenticationFailed { provider, error } => {
                    let error_payload = ErrorPayload {
                        category: "auth".to_string(),
                        message: format!("Authentication failed: {}", provider),
                        details: Some(error),
                        timestamp: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs(),
                    };
                    let _ = app.emit("lighty://error", error_payload);
                }
                _ => {}
            }
        }
    }
}

async fn handle_java_event(
    app: &AppHandle,
    event: lighty_launcher::event::JavaEvent,
    last_emit: &Arc<Mutex<Instant>>,
    progress_state: &Arc<Mutex<ProgressState>>,
    throttle: Duration,
) {
    use lighty_launcher::event::JavaEvent;

    match event {
        JavaEvent::JavaDownloadStarted {
            distribution,
            version,
            total_bytes,
        } => {
            // Réinitialiser les compteurs
            let mut state = progress_state.lock().await;
            state.java_total = total_bytes;
            state.java_downloaded = 0;
            drop(state);

            let payload = DownloadProgressPayload {
                phase: DownloadPhase::Java,
                current_bytes: 0,
                total_bytes,
                percentage: 0,
                message: format!("Downloading Java {} {}", distribution, version),
                instance_name: String::new(), // Java est global, pas lié à une instance
            };
            let _ = app.emit("lighty://download-progress", payload);
        }

        JavaEvent::JavaDownloadProgress { bytes } => {
            // Accumuler les bytes téléchargés
            let mut state = progress_state.lock().await;
            state.java_downloaded += bytes;
            let downloaded = state.java_downloaded;
            let total = state.java_total;
            drop(state);

            // Throttling
            let mut last = last_emit.lock().await;
            if last.elapsed() >= throttle {
                *last = Instant::now();
                drop(last);

                let percentage = if total > 0 {
                    ((downloaded as f64 / total as f64) * 100.0) as u8
                } else {
                    0
                };

                let payload = DownloadProgressPayload {
                    phase: DownloadPhase::Java,
                    current_bytes: downloaded,
                    total_bytes: total,
                    percentage,
                    message: format!(
                        "Downloading Java: {} / {} MB",
                        downloaded / 1_000_000,
                        total / 1_000_000
                    ),
                    instance_name: String::new(),
                };

                info!(
                    "Java download progress: {}/{} bytes ({}%)",
                    downloaded, total, percentage
                );
                let _ = app.emit("lighty://download-progress", payload);
            }
        }

        JavaEvent::JavaExtractionStarted {
            distribution,
            version,
        } => {
            let payload = DownloadProgressPayload {
                phase: DownloadPhase::Extracting,
                current_bytes: 0,
                total_bytes: 0,
                percentage: 0,
                message: format!("Extracting Java {} {}", distribution, version),
                instance_name: String::new(),
            };
            let _ = app.emit("lighty://download-progress", payload);
        }

        JavaEvent::JavaExtractionProgress {
            files_extracted,
            total_files,
        } => {
            let percentage = if total_files > 0 {
                ((files_extracted as f64 / total_files as f64) * 100.0) as u8
            } else {
                0
            };

            let payload = DownloadProgressPayload {
                phase: DownloadPhase::Extracting,
                current_bytes: files_extracted as u64,
                total_bytes: total_files as u64,
                percentage,
                message: format!(
                    "Extracting Java: {} / {} files",
                    files_extracted, total_files
                ),
                instance_name: String::new(),
            };
            let _ = app.emit("lighty://download-progress", payload);
        }

        JavaEvent::JavaExtractionCompleted { binary_path, .. } => {
            let payload = LaunchStatusPayload {
                status: LaunchStatus::Installing,
                phase: format!("Java ready: {}", binary_path),
                instance_name: String::new(),
                pid: 0,
            };
            let _ = app.emit("lighty://launch-status", payload);
        }

        _ => {}
    }
}

async fn handle_launch_event(
    app: &AppHandle,
    event: lighty_launcher::event::LaunchEvent,
    last_emit: &Arc<Mutex<Instant>>,
    progress_state: &Arc<Mutex<ProgressState>>,
    throttle: Duration,
) {
    use lighty_launcher::event::LaunchEvent;

    match event {
        LaunchEvent::IsInstalled { .. } => {
            // Ne pas émettre d'événement pour IsInstalled pendant le lancement
            // Cela évite le "pop" entre les statuts lors d'un relancement
            // Le statut initial est déjà géré par check_installation au montage
        }

        LaunchEvent::InstallStarted {
            version,
            total_bytes,
        } => {
            // Réinitialiser les compteurs
            let mut state = progress_state.lock().await;
            state.install_total = total_bytes;
            state.install_downloaded = 0;
            state.current_instance = version.clone();
            drop(state);

            let payload = DownloadProgressPayload {
                phase: DownloadPhase::Game,
                current_bytes: 0,
                total_bytes,
                percentage: 0,
                message: format!("Installing {}", version),
                instance_name: version.clone(),
            };
            let _ = app.emit("lighty://download-progress", payload);

            let status = LaunchStatusPayload {
                status: LaunchStatus::Downloading,
                phase: "Downloading game files".to_string(),
                instance_name: version,
                pid: 0,
            };
            let _ = app.emit("lighty://launch-status", status);
        }

        LaunchEvent::InstallProgress { bytes } => {
            // Accumuler les bytes téléchargés pour chaque fichier
            let mut state = progress_state.lock().await;
            state.install_downloaded += bytes;
            let downloaded = state.install_downloaded;
            let total = state.install_total;
            let instance = state.current_instance.clone();
            drop(state);

            // Throttling critique pour ne pas spam
            let mut last = last_emit.lock().await;
            if last.elapsed() >= throttle {
                *last = Instant::now();
                drop(last);

                let percentage = if total > 0 {
                    ((downloaded as f64 / total as f64) * 100.0) as u8
                } else {
                    0
                };

                let payload = DownloadProgressPayload {
                    phase: DownloadPhase::Game,
                    current_bytes: downloaded,
                    total_bytes: total,
                    percentage,
                    message: format!(
                        "Downloaded: {} / {} MB",
                        downloaded / 1_000_000,
                        total / 1_000_000
                    ),
                    instance_name: instance,
                };

                info!(
                    "Game download progress: {}/{} bytes ({}%)",
                    downloaded, total, percentage
                );
                let _ = app.emit("lighty://download-progress", payload);
            }
        }

        LaunchEvent::InstallCompleted {
            version,
            total_bytes,
        } => {
            let payload = DownloadProgressPayload {
                phase: DownloadPhase::Game,
                current_bytes: total_bytes,
                total_bytes,
                percentage: 100,
                message: format!("Installation complete: {}", version),
                instance_name: version,
            };
            let _ = app.emit("lighty://download-progress", payload);
        }

        LaunchEvent::Launching { version } => {
            let payload = LaunchStatusPayload {
                status: LaunchStatus::Installing,
                phase: "Starting game...".to_string(),
                instance_name: version,
                pid: 0,
            };
            let _ = app.emit("lighty://launch-status", payload);
        }

        LaunchEvent::Launched { version, pid } => {
            let payload = LaunchStatusPayload {
                status: LaunchStatus::Running,
                phase: "Game launched".to_string(),
                instance_name: version,
                pid: pid,
            };
            let _ = app.emit("lighty://launch-status", payload);
        }

        LaunchEvent::NotLaunched { version, error } => {
            let error_payload = ErrorPayload {
                category: "launch".to_string(),
                message: format!("Failed to launch {}", version),
                details: Some(error),
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            };
            let _ = app.emit("lighty://error", error_payload);

            let status = LaunchStatusPayload {
                status: LaunchStatus::Failed,
                phase: "Launch failed".to_string(),
                instance_name: version,
                pid: 0,
            };
            let _ = app.emit("lighty://launch-status", status);
        }

        _ => {}
    }
}

async fn handle_core_event(app: &AppHandle, event: lighty_launcher::event::CoreEvent) {
    use lighty_launcher::event::CoreEvent;

    match event {
        CoreEvent::ExtractionStarted {
            archive_type,
            file_count,
            ..
        } => {
            let payload = LaunchStatusPayload {
                status: LaunchStatus::Installing,
                phase: format!("Extracting {} ({} files)", archive_type, file_count),
                instance_name: String::new(),
                pid: 0,
            };
            let _ = app.emit("lighty://launch-status", payload);
        }

        CoreEvent::ExtractionProgress {
            files_extracted,
            total_files,
        } => {
            let percentage = if total_files > 0 {
                ((files_extracted as f64 / total_files as f64) * 100.0) as u8
            } else {
                0
            };

            let payload = DownloadProgressPayload {
                phase: DownloadPhase::Extracting,
                current_bytes: files_extracted as u64,
                total_bytes: total_files as u64,
                percentage,
                message: format!("Extracting: {} / {} files", files_extracted, total_files),
                instance_name: String::new(), // Core extraction pas lié à instance spécifique
            };
            let _ = app.emit("lighty://download-progress", payload);
        }

        CoreEvent::ExtractionCompleted {
            files_extracted, ..
        } => {
            let payload = DownloadProgressPayload {
                phase: DownloadPhase::Extracting,
                current_bytes: files_extracted as u64,
                total_bytes: files_extracted as u64,
                percentage: 100,
                message: format!("Extraction complete: {} files", files_extracted),
                instance_name: String::new(),
            };
            let _ = app.emit("lighty://download-progress", payload);
        }
    }
}
