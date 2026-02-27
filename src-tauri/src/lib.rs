mod commands;
mod events;
mod runners;
mod types;

use tauri::Manager;
use lighty_launcher::{core::AppState, event::EventBus};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(_app_state: AppState) -> anyhow::Result<()> {
    tracing::info!("🏁 Démarrage du launcher");

    let event_bus = EventBus::new(1000);
    let mut builder =
        tauri::Builder::default().plugin(tauri_plugin_updater::Builder::new().build());

    builder = runners::setup(builder);

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
    builder = builder.manage(event_bus.clone());

    let app = builder
        .build(tauri::generate_context!())
        .unwrap_or_else(|e| {
            println!("Erreur Tauri: {:?}", e);
            panic!("error while building tauri application");
        });
    println!("Tauri build OK");

    let app_handle = app.handle().clone();
    events::spawn_event_listener(app_handle, event_bus.clone());

    // use std::sync::{Arc, Mutex};
    use tauri::{RunEvent, WindowEvent};

    // // On utilise un Arc<Mutex<bool>> pour suivre si Minecraft est lancé
    // let is_mc_running: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));

    // // Vérifie la présence d'une instance Minecraft au démarrage
    // let instance_id = "minecraft".to_string(); // ou autre id selon logique
    // let instance_present = commands::game::get_instance(instance_id.clone()).unwrap_or(false);
    // println!("Instance présente au démarrage : {}", instance_present);

    app.run(move |_app_handle, event| {
        match &event {
            RunEvent::WindowEvent { label, event, .. } => {
                if label == "main" {
                    if let WindowEvent::CloseRequested { api: _, .. } = event {
                    //     // let running = is_mc_running.lock().unwrap();
                    //     // if *running {
                    //     //     // Si Minecraft est lancé, on cache la fenêtre
                    //     //     if let Some(window) = app_handle.get_webview_window("main") {
                    //     //         let _ = window.hide();
                    //     //     }
                    //     //     api.prevent_close();
                    //     // } else {
                    //     //     // Sinon, on quitte normalement
                    //     //     // rien à faire, la fenêtre se ferme
                    //     // }
                    }
                }
            }
            _ => {}
        }
    });

    Ok(())
}
