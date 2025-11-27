use crate::commands::get_screenshots_dir;
use crate::tracker::env::load_config;
use rust_dotenv::dotenv::DotEnv;
use tauri::{Emitter, Manager};
use tauri_plugin_autostart::MacosLauncher;

mod commands;
mod tracker;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let dotenv = DotEnv::new("");
    let config = load_config(&dotenv);
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--autostart"]),
        ))
        .manage(config)
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                window.hide().unwrap();
                window.app_handle().emit("lock", ()).unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .setup(|app| {
            let handle = app.handle();
            tracker::start_tracking(handle);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_screenshots_dir])
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
