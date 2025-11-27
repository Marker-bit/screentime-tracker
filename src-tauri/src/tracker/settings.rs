use std::sync::Arc;

use serde_json::Value;
use tauri::Wry;
use tauri_plugin_store::Store;

fn parse_settings(settings_str: &str) -> Result<Value, String> {
    serde_json::from_str(settings_str)
        .map_err(|e| format!("Failed to parse settings object: {}", e.to_string()))
}

pub fn get_settings(store: Arc<Store<Wry>>) -> Result<Value, String> {
    let settings_value = store.get("settings").unwrap();
    let settings_str = settings_value
        .as_str()
        .ok_or("Failed to get settings string")?;
    Ok(parse_settings(settings_str)?)
}
