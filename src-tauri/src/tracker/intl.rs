#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Language {
    English,
    Russian,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MessageKey {
    BreakNotificationTitle,
    BreakNotificationContent,
    StopNotificationTitle,
    StopNotificationContent,
    Desktop,
    // Add more static keys as needed
}

pub fn get_lang(lang: &str) -> Language {
    match lang {
        "ru" => Language::Russian,
        _ => Language::English,
    }
}

pub fn translate(lang: Language, key: MessageKey) -> &'static str {
    match (lang, key) {
        (Language::English, MessageKey::BreakNotificationTitle) => "BreakNotificationTitle!",
        (Language::English, MessageKey::BreakNotificationContent) => "BreakNotificationContent!",
        (Language::English, MessageKey::Desktop) => "Desktop",
        (Language::English, MessageKey::StopNotificationContent) => "Рабочий стол",
        (Language::English, MessageKey::StopNotificationTitle) => "Рабочий стол",

        (Language::Russian, MessageKey::BreakNotificationTitle) => "Пора сделать перерыв!",
        (Language::Russian, MessageKey::BreakNotificationContent) => "Нужно заняться чем-то другим",
        (Language::Russian, MessageKey::Desktop) => "Рабочий стол",
        (Language::Russian, MessageKey::StopNotificationContent) => "Рабочий стол",
        (Language::Russian, MessageKey::StopNotificationTitle) => "Рабочий стол",
    }
}
