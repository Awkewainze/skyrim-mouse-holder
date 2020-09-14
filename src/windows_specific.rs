extern crate libc;
extern crate user32;
extern crate winapi;

use std::collections::HashSet;
use std::ffi::CString;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use enigo::*;
use user32::{
    GetForegroundWindow, GetSystemMetrics, GetWindowTextLengthW, GetWindowTextW, MessageBoxA,
};
use winapi::winuser::MB_OK;
use winapi::HWND;

use crate::window_structs::{Point, ScreenDimensions};

const SYS_METRIC_WIDTH: i32 = 0;
const SYS_METRIC_HEIGHT: i32 = 1;

#[cfg(target_os = "windows")]
pub unsafe fn open_message_box() {
    let lp_text = CString::new("Mouse will now center as long as this dialogue is open and only while Skyrim or Skyrim SE is open.").unwrap();
    let lp_caption = CString::new("Skyrim Mouse Holder").unwrap();

    MessageBoxA(
        std::ptr::null_mut(),
        lp_text.as_ptr(),
        lp_caption.as_ptr(),
        MB_OK,
    );
}

#[cfg(target_os = "windows")]
pub unsafe fn get_screen_dimensions() -> ScreenDimensions {
    ScreenDimensions {
        width: GetSystemMetrics(SYS_METRIC_WIDTH) as usize,
        height: GetSystemMetrics(SYS_METRIC_HEIGHT) as usize,
    }
}

#[cfg(target_os = "windows")]
pub unsafe fn get_window_name(window_handle: HWND) -> Box<str> {
    let title_length = GetWindowTextLengthW(window_handle) + 1;
    let mut title_vec: Vec<u16> = Vec::with_capacity(title_length as usize);
    let read_len = GetWindowTextW(window_handle, title_vec.as_mut_ptr(), title_length);
    if read_len > 0 {
        title_vec.set_len((read_len) as usize);
        return String::from_utf16_lossy(&title_vec).into_boxed_str();
    }
    Box::from("")
}

#[cfg(target_os = "windows")]
pub unsafe fn get_current_window_name() -> Box<str> {
    return get_window_name(GetForegroundWindow());
}

#[cfg(target_os = "windows")]
pub unsafe fn loop_until_interrupt(stop: Arc<AtomicBool>, titles_to_center_on: HashSet<&str>) {
    let mut enigo = Enigo::new();
    let screen_dimensions = get_screen_dimensions();
    let midpoint = Point {
        x: screen_dimensions.width as i32 / 2,
        y: screen_dimensions.height as i32 / 2,
    };

    while !stop.load(Ordering::Relaxed) {
        if titles_to_center_on.contains(get_current_window_name().into_string().as_str()) {
            enigo.mouse_move_to(midpoint.x, midpoint.y);
        }
        sleep(Duration::from_millis(100));
    }
}
