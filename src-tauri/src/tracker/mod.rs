pub mod apps_time_map;
pub mod email;
pub mod env;
mod models;
mod tray;
mod usage_info;
mod utils;

use chrono::{Local, Timelike, Utc};
pub use models::CurrentActivity;
use serde_json::Value;
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
        let mut triggered_today = false;
        let mut sent_break_notification_today = false;
        let mut sent_stop_notification_today = false;

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
            let now = Local::now();
            if now.hour() == 23 && now.minute() == 34 && !sent_break_notification_today {
                app_clone
                    .notification()
                    .builder()
                    .title("Пора сделать перерыв!")
                    .body("Нужно заняться чем-то другим")
                    .show()
                    .unwrap();
                sent_break_notification_today = true;
            } else if now.hour() != 23 {
                sent_break_notification_today = false;
            }
            if now.hour() == 22 && now.minute() == 31 && !triggered_today {
                let email_content = generate_email_content(
                    Utc::now().date_naive(),
                    optimistic_local_time / 1000 / 60,
                    apps_time_map_optimistic.clone(),
                );
                let store = app_clone.store("store.json").unwrap();
                let settings_value = store.get("settings").unwrap();
                let settings_str = settings_value.as_str().unwrap();
                let v: Value = serde_json::from_str(settings_str).unwrap();
                let email = v
                    .get("state")
                    .expect("Failed to get state")
                    .get("settings")
                    .expect("Failed to get settings")
                    .get("parentEmail")
                    .expect("Failed to get parent email")
                    .as_str()
                    .expect("Failed to get parent email as str");
                println!("Sending email to {}:\n{:#?}", email, email_content);
                if let Err(e) = send_email(
                    app_clone.state::<AppConfig>().inner(),
                    email_content,
                    email.to_string(),
                ) {
                    println!("Failed to send email: {}", e);
                };
                triggered_today = true;
            } else if now.hour() != 22 {
                triggered_today = false;
            }
            app_clone
                .emit("apps-info", apps_time_map_optimistic.clone())
                .expect("Failed to emit an event");

            // println!("idle_time_minutes = {}", get_idle_time_minutes());

            // if get_idle_time_minutes() < 5 {
            //     stats.active_minutes_today += 1;
            //     println!("active {} min", stats.active_minutes_today);
            // } else {
            //     println!("idle...");
            // }

            // let today = Local::now().format("%Y-%m-%d").to_string();
            // if today != stats.last_saved {
            //     stats.active_minutes_today = 0;
            //     stats.last_saved = today.clone();
            // }

            // save_stats(&stats);
        }
    });
}
