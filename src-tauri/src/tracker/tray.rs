use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager, Wry,
};

pub fn create_tray(app: &AppHandle) -> Menu<Wry> {
    let time_i = MenuItem::with_id(app, "time", "5 мин сегодня", false, None::<&str>).unwrap();
    let quit_i = MenuItem::with_id(app, "quit", "Выйти", true, None::<&str>).unwrap();
    let menu = Menu::with_items(app, &[&time_i, &quit_i]).unwrap();

    TrayIconBuilder::new()
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_tray_icon_event(|icon, event| match event {
            TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } => {
                let app = icon.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    if window
                        .is_visible()
                        .expect("Failed to get window visible status")
                    {
                        let _ = window.hide();
                        app.emit("lock", ()).unwrap();
                    } else {
                        let _ = window.unminimize();
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
            _ => {}
        })
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "quit" => {
                app.exit(0);
            }
            _ => {
                println!("menu item {:?} not handled", event.id);
            }
        })
        .build(app)
        .expect("Failed to build tray");
    return menu;
}
