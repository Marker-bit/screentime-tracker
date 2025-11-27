pub mod apps_time_map;
pub mod email;
pub mod env;
mod intl;
mod models;
pub mod screenshot;
mod settings;
mod tray;
mod usage_info;
mod utils;

use chrono::{DateTime, Local, NaiveDate, Timelike, Utc};
pub use models::CurrentActivity;
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_store::StoreExt;
pub use utils::get_window_title;

use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};
use tokio::time::sleep;
use usage_info::{get_last_input_info, get_tick_count};

use crate::tracker::{
    apps_time_map::AppsTimeMap,
    email::{generate_email_content, send_email},
    env::AppConfig,
    intl::{get_lang, translate, MessageKey},
    screenshot::take_all_screenshots,
    settings::get_settings,
    tray::create_tray,
};

pub fn start_tracking(app: &AppHandle) {
    let app_clone = app.clone();
    let menu = create_tray(app);

    tauri::async_runtime::spawn(async move {
        println!("Started tracking");
        let current_tick_count = get_tick_count();
        let program_start_time = current_tick_count;
        let mut prev_move_time = current_tick_count;
        let mut current_activity = CurrentActivity {
            app_name: get_window_title(),
            start_time: None,
        };
        let mut total_time: u32 = 0;
        let mut apps_time_map = AppsTimeMap::new();
        let mut reset_date: Option<NaiveDate> = None;
        let mut email_sent_date: Option<NaiveDate> = None;
        let mut break_notification_date: Option<NaiveDate> = None;
        let mut screenshot_datetime: Option<DateTime<Local>> = None;
        let mut stop_notification_date: Option<NaiveDate> = None;

        loop {
            sleep(Duration::from_millis(250)).await;
            let active_window_title = get_window_title();
            let current_tick_count = get_tick_count();
            match get_last_input_info(program_start_time) {
                Ok(info) => {
                    if info.dwTime != prev_move_time && current_activity.start_time.is_none() {
                        current_activity.start_time = Some(info.dwTime);
                    }
                    if (current_tick_count - info.dwTime > 50000
                        || current_activity.app_name.clone() != active_window_title)
                        && current_activity.start_time.is_some()
                    {
                        let start_time = current_activity.start_time.unwrap();
                        if info.dwTime >= start_time {
                            total_time += (info.dwTime - start_time) as u32;
                            apps_time_map
                                .add(current_activity.app_name.clone(), info.dwTime - start_time);
                            current_activity.start_time = None;
                            current_activity.app_name = active_window_title;
                        }
                    }
                    prev_move_time = info.dwTime;
                }
                Err(_) => {
                    println!("Failed to get last input info");
                }
            };
            let optimistic_local_time = total_time
                + (if let Some(start_time) = current_activity.start_time {
                    current_tick_count - start_time
                } else {
                    0
                });
            let _ = menu
                .get("time")
                .expect("Didn't find menu element named \"time\"")
                .as_menuitem()
                .expect("\"time\" should be a menu item")
                .set_text(format!(
                    "{:.1} мин сегодня",
                    optimistic_local_time as f32 / 1000f32 / 60f32
                ));
            app_clone
                .emit("total-time", optimistic_local_time)
                .expect("Failed to emit an event");
            let mut apps_time_map_optimistic = apps_time_map.clone();
            apps_time_map_optimistic.add(
                current_activity.app_name.clone(),
                if let Some(start_time) = current_activity.start_time {
                    current_tick_count - start_time
                } else {
                    0
                },
            );
            let store = app_clone.store("store.json").unwrap();
            let settings = get_settings(store).unwrap();
            let now = Local::now();

            let needs_screenshot = screenshot_datetime.is_none()
                || screenshot_datetime.as_ref().map_or(true, |dt| {
                    dt.date_naive() != now.date_naive() || dt.hour() != now.hour()
                });

            if needs_screenshot {
                match take_all_screenshots() {
                    Ok(_) => println!("Screenshots taken successfully"),
                    Err(e) => {
                        println!("Failed to take screenshots: {}", e);
                    }
                };
                screenshot_datetime = Some(now);
            }
            if now.hour() == 9
                && now.minute() == 42
                && break_notification_date != Some(now.date_naive())
            {
                let lang = get_lang(
                    settings
                        .pointer("/state/settings/language")
                        .and_then(|v| v.as_str())
                        .expect("Failed to get language"),
                );
                app_clone
                    .notification()
                    .builder()
                    .title(translate(lang, MessageKey::BreakNotificationTitle))
                    .body(translate(lang, MessageKey::BreakNotificationContent))
                    .show()
                    .unwrap();
                break_notification_date = Some(now.date_naive());
            }
            if now.hour() == 22 && now.minute() == 31 && email_sent_date != Some(now.date_naive()) {
                let email_content = generate_email_content(
                    Utc::now().date_naive(),
                    optimistic_local_time / 1000 / 60,
                    apps_time_map_optimistic.clone(),
                );
                let email = settings
                    .pointer("/state/settings/parentEmail")
                    .and_then(|v| v.as_str())
                    .expect("Failed to get parent email");
                println!("Sending email to {}:\n{:#?}", email, email_content);
                if let Err(e) = send_email(
                    app_clone.state::<AppConfig>().inner(),
                    email_content,
                    email.to_string(),
                ) {
                    println!("Failed to send email: {}", e);
                };
                email_sent_date = Some(now.date_naive());
            }
            app_clone
                .emit("apps-info", apps_time_map_optimistic.clone())
                .expect("Failed to emit an event");
            if now.hour() == 0 && now.minute() == 0 && reset_date != Some(now.date_naive()) {
                total_time = 0;
                reset_date = Some(now.date_naive());
            }
        }
    });
}
