use crate::tracker::intl::{Language, MessageKey, translate};

use super::usage_info::get_active_window_title;

pub fn get_window_title(lang: Language) -> String {
    get_active_window_title().unwrap_or(translate(lang, MessageKey::Desktop).to_string())
}
