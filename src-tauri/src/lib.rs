mod commands;

use commands::accounts::{init_vault_if_needed, VaultState};
use lighty_launcher::{
    core::AppState,
    event::{Event, EventBus, EventTryReceiveError},
};
use tauri::{Emitter, Manager};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

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

    let event_bus = EventBus::new(1000);
    let mut receiver = event_bus.subscribe();

    let mut builder =
        tauri::Builder::default().plugin(tauri_plugin_updater::Builder::new().build());

    builder = builder.manage(VaultState::default());

    builder = builder.setup(|app| {
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

        Ok(())
    });

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }));
    }

    builder = builder.plugin(tauri_plugin_window_state::Builder::new().build());
    builder = builder.plugin(tauri_plugin_opener::init());
    builder = builder.manage(event_bus);
    builder = builder.invoke_handler(commands::handler());
    let app = builder
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(move |app_handle, event| {
        if matches!(event, tauri::RunEvent::MainEventsCleared) {
            loop {
                match receiver.try_next() {
                    Ok(event) => match event {
                        Event::ConsoleOutput(e) => {
                            let payload = serde_json::json!({
                                "line": e.line,
                                "stream": format!("{:?}", e.stream)
                            });
                            let _ = app_handle.emit("launcher:console", payload);
                        }
                        Event::InstanceExited(e) => {
                            let _ = app_handle.emit("launcher:exit", e.exit_code);
                        }
                        _ => {}
                    },
                    Err(EventTryReceiveError::Empty) => break,
                    Err(EventTryReceiveError::Lagged { .. }) => continue,
                    Err(EventTryReceiveError::BusDropped) => break,
                }
            }
        }
    });
}
