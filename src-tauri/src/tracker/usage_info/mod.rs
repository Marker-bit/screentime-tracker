use windows::Win32::Foundation::HWND;
use windows::Win32::System::SystemInformation::GetTickCount;
use windows::Win32::UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO};
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW};

pub fn get_active_window_title() -> Option<String> {
    unsafe {
        let hwnd: HWND = GetForegroundWindow();
        let mut buf = [0u16; 512];
        let len = GetWindowTextW(hwnd, &mut buf);
        if len > 0 {
            Some(String::from_utf16_lossy(&buf[..len as usize]))
        } else {
            None
        }
    }
}

pub fn get_tick_count() -> u32 {
    unsafe { GetTickCount() }
}

pub fn get_last_input_info(start_time: u32) -> Result<LASTINPUTINFO, ()> {
    unsafe {
        let mut info = LASTINPUTINFO {
            cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32,
            dwTime: 0,
        };

        if GetLastInputInfo(&mut info).as_bool() {
            if start_time > info.dwTime {
                info.dwTime = start_time;
            }
            Ok(info)
        } else {
            Err(())
        }
    }
}
