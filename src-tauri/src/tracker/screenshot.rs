use chrono::Local;
use dirs::picture_dir;
use std::fs;
use std::path::PathBuf;
use xcap::Monitor;

#[cfg(windows)]
pub const APP_SCREENSHOTS_SUBDIR: &str = r"ScreentimeTracker\screenshots";
#[cfg(not(windows))]
pub const APP_SCREENSHOTS_SUBDIR: &str = "ScreentimeTracker/screenshots";

pub fn get_screenshots_dir() -> Result<PathBuf, String> {
    let screenshots_dir: PathBuf = picture_dir()
        .ok_or_else(|| "No pictures directory available".to_string())?
        .join(APP_SCREENSHOTS_SUBDIR);

    Ok(screenshots_dir)
}

fn normalized(filename: String) -> String {
    filename.replace(['|', '\\', ':', '*', '?', '"', '<', '>', '/'], "")
}

pub fn take_all_screenshots() -> Result<(), String> {
    let current_date = Local::now();
    let monitors = Monitor::all().map_err(|e| format!("Failed to get monitors: {}", e))?;

    let screenshots_dir = get_screenshots_dir()?;

    println!("Folder for screenshots: {:#?}", screenshots_dir);

    fs::create_dir_all(&screenshots_dir).map_err(|e| {
        format!(
            "Failed to create screenshots dir {:?}: {}",
            screenshots_dir, e
        )
    })?;

    for (monitor_index, monitor) in monitors.iter().enumerate() {
        let image = monitor
            .capture_image()
            .map_err(|e| format!("Failed to capture monitor {}: {}", monitor_index, e))?;

        let monitor_name = monitor
            .name()
            .unwrap_or_else(|_err| format!("monitor-{}", monitor_index));

        let filename = format!(
            "{}-{}.png",
            current_date.format("%Y-%m-%d_%H-%M"),
            normalized(monitor_name)
        );

        image
            .save(screenshots_dir.join(&filename))
            .map_err(|e| format!("Failed to save {:?}: {}", filename, e))?;
    }
    Ok(())
}
