#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Language {
    English,
    Russian,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MessageKey {
    BreakNotificationTitle,
    BreakNotificationContent,
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
        (Language::English, MessageKey::BreakNotificationTitle) => "It's time to take a break!",
        (Language::English, MessageKey::BreakNotificationContent) => "You should do something else",
        (Language::English, MessageKey::Desktop) => "Desktop",

        (Language::Russian, MessageKey::BreakNotificationTitle) => "Пора сделать перерыв!",
        (Language::Russian, MessageKey::BreakNotificationContent) => "Нужно заняться чем-то другим",
        (Language::Russian, MessageKey::Desktop) => "Рабочий стол",
    }
}
