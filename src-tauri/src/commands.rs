use crate::tracker::screenshot;

#[tauri::command]
pub fn get_screenshots_dir() -> Result<String, String> {
    screenshot::get_screenshots_dir().map(|pathbuf| pathbuf.to_str().unwrap().to_string())
}
