use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UsageStats {
    pub active_minutes_today: u64,
    pub last_saved: String,
}

#[derive(Debug)]
pub struct CurrentActivity {
    pub app_name: String,
    pub start_time: Option<u32>,
}
