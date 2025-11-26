use super::usage_info::get_active_window_title;

pub fn get_window_title() -> String {
    get_active_window_title().unwrap_or("Рабочий стол".to_string())
}
