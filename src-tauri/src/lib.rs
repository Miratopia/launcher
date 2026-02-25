mod commands;
mod events;
mod types;

use commands::accounts::{init_vault_if_needed, VaultState};
use lighty_launcher::{core::AppState, event::EventBus};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconEvent};
use tauri::Manager;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
};
use tracing_subscriber::prelude::*;

fn init_tracing() -> tracing_appender::non_blocking::WorkerGuard {
    let log_dir = std::env::temp_dir().join("miratopia-launcher");
    let _ = std::fs::create_dir_all(&log_dir);
    let log_path = log_dir.join("launcher.log");
    let log_file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .expect("failed to open log file");

    let (non_blocking, guard) = tracing_appender::non_blocking(log_file);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(std::io::stdout)
                .with_ansi(true),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(non_blocking)
                .with_ansi(false),
        )
        .init();

    guard
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    println!("=== MIRATOPIA LAUNCHER: run() démarre ===");
    let _tracing_guard = init_tracing();
    println!("Tracing OK");

    const QUALIFIER: &str = "fr";
    const ORGANIZATION: &str = ".miratopia";
    const APPLICATION: &str = "";
    const VAULT_PASSWORD: &str = "dev-vault-password";

    let _app = AppState::new(
        QUALIFIER.to_string(),
        ORGANIZATION.to_string(),
        APPLICATION.to_string(),
    )
    .map_err(|e| format!("{:?}", e));
    println!("AppState OK");

    let event_bus = EventBus::new(1000);
    println!("EventBus OK");
    // let mut receiver = event_bus.subscribe();

    let mut builder =
        tauri::Builder::default().plugin(tauri_plugin_updater::Builder::new().build());
    println!("Builder OK");

    builder = builder.manage(VaultState::default());
    println!("VaultState OK");

    builder = builder.setup(|app| {
        println!("Setup closure OK");
        let salt_path = app
            .path()
            .app_local_data_dir()
            .expect("could not resolve app local data path")
            .join("salt.txt");

        app.handle()
            .plugin(tauri_plugin_stronghold::Builder::with_argon2(&salt_path).build())?;

        let vault_state = app.state::<VaultState>();
        if let Err(err) = init_vault_if_needed(&app.handle(), &vault_state, VAULT_PASSWORD) {
            tracing::error!(%err, "vault init failed");
        }

        let quit_i = MenuItem::with_id(app, "quit", "Quitter", true, None::<&str>)?;
        let console_i = MenuItem::with_id(app, "console", "Console", true, None::<&str>)?;
        let menu = Menu::with_items(app, &[&console_i, &quit_i])?;

        TrayIconBuilder::new()
            .icon(app.default_window_icon().unwrap().clone())
            .on_tray_icon_event(|tray, event| match event {
                TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                } => {
                    let app = tray.app_handle();

                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.unminimize();
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                _ => {}
            })
            .menu(&menu)
            .show_menu_on_left_click(true)
            .on_menu_event(|app, event| match event.id.as_ref() {
                "quit" => {
                    println!("quit menu item was clicked");
                    app.exit(0);
                }
                _ => {
                    println!("menu item {:?} not handled", event.id);
                }
            })
            .build(app)?;

        Ok(())
    });
    println!("Setup OK");

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }));
        println!("Single instance plugin OK");
    }

    builder = builder.plugin(tauri_plugin_window_state::Builder::new().build());
    builder = builder.plugin(tauri_plugin_opener::init());
    // builder = builder.manage(event_bus.clone());
    builder = builder.invoke_handler(commands::handler());
    println!("Plugins and invoke_handler OK");

    let app = builder
        .build(tauri::generate_context!())
        .unwrap_or_else(|e| {
            println!("Erreur Tauri: {:?}", e);
            panic!("error while building tauri application");
        });
    println!("Tauri build OK");

    let app_handle = app.handle().clone();
    events::spawn_event_listener(app_handle, event_bus.clone());
    app.manage(event_bus);
    println!("Event listener and manage OK");

    use std::sync::{Arc, Mutex};
    use tauri::{RunEvent, WindowEvent};

    // On utilise un Arc<Mutex<bool>> pour suivre si Minecraft est lancé
    let is_mc_running: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));

    // Vérifie la présence d'une instance Minecraft au démarrage
    let instance_id = "minecraft".to_string(); // ou autre id selon logique
    let instance_present = commands::game::get_instance(instance_id.clone()).unwrap_or(false);
    println!("Instance présente au démarrage : {}", instance_present);

    app.run(move |app_handle, event| {
        match &event {
            RunEvent::WindowEvent { label, event, .. } => {
                if label == "main" {
                    if let WindowEvent::CloseRequested { api, .. } = event {
                        let running = is_mc_running.lock().unwrap();
                        if *running {
                            // Si Minecraft est lancé, on cache la fenêtre
                            if let Some(window) = app_handle.get_webview_window("main") {
                                let _ = window.hide();
                            }
                            api.prevent_close();
                        } else {
                            // Sinon, on quitte normalement
                            // rien à faire, la fenêtre se ferme
                        }
                    }
                }
            }
            _ => {}
        }
    });

    // Buffer pour les progress et timer
    // use std::time::{Duration, Instant};
    // let mut last_java_emit = Instant::now();
    // let mut last_launcher_emit = Instant::now();
    // let mut java_progress: Option<u64> = None;
    // let mut launcher_progress: Option<u64> = None;

    // app.run(move |app_handle, event| {
    //     if matches!(event, tauri::RunEvent::MainEventsCleared) {
    //         use std::thread::sleep;
    //         loop {
    //             match receiver.try_next() {
    //                 Ok(event) => match event {
    //                     Event::Java(JavaEvent::JavaDownloadStarted {
    //                         distribution,
    //                         version,
    //                         total_bytes,
    //                     }) => {
    //                         let payload = serde_json::json!({
    //                             "distribution": distribution,
    //                             "version": version,
    //                             "total_bytes": total_bytes,
    //                         });
    //                         let _ = app_handle.emit("launcher:java-download-started", payload);
    //                     }
    //                     Event::Java(JavaEvent::JavaDownloadProgress { bytes }) => {
    //                         java_progress = Some(bytes);
    //                     }
    //                     Event::Java(JavaEvent::JavaDownloadCompleted {
    //                         distribution,
    //                         version,
    //                     }) => {
    //                         let payload = serde_json::json!({
    //                             "distribution": distribution,
    //                             "version": version,
    //                         });
    //                         let _ = app_handle.emit("launcher:java-download-completed", payload);
    //                     }
    //                     Event::Launch(LaunchEvent::InstallStarted {
    //                         version,
    //                         total_bytes,
    //                     }) => {
    //                         let payload = serde_json::json!({
    //                             "version": version,
    //                             "total_bytes": total_bytes,
    //                         });
    //                         let _ = app_handle.emit("launcher:launcher-download-started", payload);
    //                     }
    //                     Event::Launch(LaunchEvent::InstallProgress { bytes }) => {
    //                         launcher_progress = Some(bytes);
    //                     }
    //                     Event::Launch(LaunchEvent::InstallCompleted {
    //                         version,
    //                         total_bytes,
    //                     }) => {
    //                         let payload = serde_json::json!({
    //                             "version": version,
    //                             "total_bytes": total_bytes,
    //                         });
    //                         let _ =
    //                             app_handle.emit("launcher:launcher-download-completed", payload);
    //                     }
    //                     Event::ConsoleOutput(e) => {
    //                         let payload = serde_json::json!({
    //                             "line": e.line,
    //                             "stream": format!("{:?}", e.stream)
    //                         });
    //                         let _ = app_handle.emit("launcher:console", payload);
    //                     }
    //                     Event::InstanceExited(e) => {
    //                         let _ = app_handle.emit("launcher:exit", e.exit_code);
    //                     }
    //                     _ => {}
    //                 },
    //                 Err(EventTryReceiveError::Empty) => break,
    //                 Err(EventTryReceiveError::Lagged { .. }) => {
    //                     tracing::warn!(
    //                         "EventBus Lagged: des événements ont été perdus (bus plein)"
    //                     );
    //                     continue;
    //                 }
    //                 Err(EventTryReceiveError::BusDropped) => break,
    //             }
    //             // Throttle strict : on émet la dernière valeur toutes les 50ms, même si elle ne change pas
    //             let now = Instant::now();
    //             if now.duration_since(last_java_emit) >= Duration::from_millis(50) {
    //                 if let Some(bytes) = java_progress {
    //                     let payload = serde_json::json!({ "bytes": bytes });
    //                     let _ = app_handle.emit("launcher:java-download-progress", payload);
    //                     last_java_emit = now;
    //                 }
    //             }
    //             if now.duration_since(last_launcher_emit) >= Duration::from_millis(50) {
    //                 if let Some(bytes) = launcher_progress {
    //                     let payload = serde_json::json!({ "bytes": bytes });
    //                     let _ = app_handle.emit("launcher:launcher-download-progress", payload);
    //                     last_launcher_emit = now;
    //                 }
    //             }
    //             sleep(Duration::from_millis(5)); // pour éviter de bloquer le CPU
    //         }
    //     }
    // });
}
