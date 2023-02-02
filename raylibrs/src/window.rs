use std::{ffi::{CString, NulError, c_int, CStr}, str::FromStr, error::Error, os::raw::{c_uint, c_void}, rc::Rc, convert::Infallible};

use glam::Vec2;
use lazy_static::lazy_static;
use raylibrs_sys::*;

/// The window. DO NOT CREATE TWO INSTANCES AT THE SAME TIME. This would be unexpected Behavior due to Raylib not supporting More then one Window.
/// The fields are currently not useful.
struct Window {
    size: Vec2,
    title: String,
}

pub enum WindowBuildError {
    NulError(NulError),
    StringErr(Infallible)
}

impl From<Infallible> for WindowBuildError {
    fn from(value: Infallible) -> Self {
        Self::StringErr(value)
    }
}

impl From<NulError> for WindowBuildError {
    fn from(value: NulError) -> Self {
        Self::NulError(value)
    }
}



impl Window {
    pub fn build(size: Vec2, title: &str) -> Result<Self, WindowBuildError> {
        unsafe{ InitWindow(size.x as i32, size.y as i32, CString::new(title)?.as_ptr());}
        Ok(Self { size: size, title: String::from_str(title)? })
    }

    pub fn should_close(self: &Self) -> bool {
        unsafe {WindowShouldClose()}
    }

    pub fn is_ready(self: &Self) -> bool {
        unsafe {IsWindowReady()}
    }

    pub fn is_fullscreen(self: &Self) -> bool {
        unsafe {IsWindowFullscreen()}
    }

    pub fn is_hidden(self: &Self) -> bool {
        unsafe {IsWindowHidden()}
    }

    pub fn is_minimized(self: &Self) -> bool {
        unsafe {IsWindowMinimized()}
    }

    pub fn is_maximized(self: &Self) -> bool {
        unsafe {IsWindowMaximized()}
    }

    pub fn is_focused(self: &Self) -> bool {
        unsafe {IsWindowFocused()}
    }

    pub fn is_resized(self: &Self) -> bool {
        unsafe {IsWindowResized()}
    }
    
    pub fn is_state(self: &Self, flag: ConfigFlags) -> bool {
        unsafe {IsWindowState(flag as c_uint)}
    }

    pub fn set_state(self: &Self, flag: ConfigFlags) {
        unsafe {SetWindowState(flag as c_uint)}
    }

    pub fn clear_state(self: &Self, flag: ConfigFlags) {
        unsafe {ClearWindowState(flag as c_uint)}
    }

    pub fn toggle_fullscreen(self: &Self) {
        unsafe {ToggleFullscreen()}
    }

    pub fn maximize(self: &Self) {
        unsafe {MaximizeWindow()}
    }

    pub fn minimize(self: &Self) {
        unsafe {MinimizeWindow()}
    }

    pub fn restore(self: &Self) {
        unsafe {RestoreWindow()}
    }

    pub fn set_icon(self: &Self) {todo!("Not done")}

    pub fn set_title(self: &mut Self, title: &str) -> Result<(), NulError> {
        unsafe {SetWindowTitle(CString::new(title)?.as_ptr());}
        self.title = title.to_string();
        Ok(())
    }

    pub fn set_pos(self: &mut Self, pos: Vec2) {
        unsafe {SetWindowPosition(pos.x as c_int, pos.y as c_int);}
    }

    pub fn set_monitor(self: &mut Self, monitor: i32) {
        unsafe {SetWindowMonitor(monitor);}
    }

    pub fn set_min_size(self: &mut Self, size: Vec2) {
        unsafe {SetWindowMinSize(size.x as c_int, size.y as c_int);}
    }

    pub fn set_size(self: &mut Self, size: Vec2) {
        unsafe {SetWindowSize(size.x as i32, size.y as i32);}
    }

    pub fn set_opacity(self: &mut Self, op: f32) {
        unsafe {SetWindowOpacity(op);}
    }

    pub fn get_handle(self: &mut Self ) -> *mut c_void {
        unsafe {GetWindowHandle()}
    }

    pub fn get_screen_width(self: &mut Self ) -> i32 {
        unsafe {GetScreenWidth()}
    }

    pub fn get_screen_height(self: &mut Self) -> i32 {
        unsafe {GetScreenHeight()}
    }

    pub fn get_monitor_count(self: &mut Self) -> i32 {
        unsafe {GetMonitorCount()}
    }

    pub fn get_current_monitor(self: &mut Self) -> i32 {
        unsafe {GetCurrentMonitor()}
    }

    pub fn get_monitor_positon(self: &mut Self, monitor: i32) -> Vec2 {
        unsafe {GetMonitorPosition(monitor).into()}
    }

    pub fn get_monitor_width(self: &mut Self, monitor: i32) -> i32 {
        unsafe {GetMonitorWidth(monitor)}
    }

    pub fn get_monitor_height(self: &mut Self, monitor: i32) -> i32 {
        unsafe {GetMonitorHeight(monitor)}
    }

    pub fn get_monitor_physical_width(self: &mut Self, monitor: i32) -> i32 {
        unsafe {GetMonitorPhysicalWidth(monitor)}
    }

    pub fn get_monitor_physical_height(self: &mut Self, monitor: i32) -> i32 {
        unsafe {GetMonitorPhysicalHeight(monitor)}
    }

    pub fn get_monitor_refreshrate(self: &mut Self, monitor: i32) -> i32 {
        unsafe {GetMonitorRefreshRate(monitor)}
    }

    pub fn get_position(self: &mut Self) -> Vec2 {
        unsafe {GetWindowPosition().into()}
    }

    pub fn get_scale_dpi(self: &mut Self) -> Vec2 {
        unsafe {GetWindowScaleDPI().into()}
    }

    pub fn get_monitor_name(self: &mut Self, monitor: i32) -> Result<&str, std::str::Utf8Error> {
        unsafe {CStr::from_ptr(GetMonitorName(monitor)).to_str()}
    }

    pub fn set_clipboard_text(self: &mut Self, text: &str) -> Result<(), NulError> {
        unsafe {SetClipboardText(CString::new(text)?.as_ptr());}
        Ok(())
    }

    pub fn get_clipboard_text(self: &mut Self) -> Result<&str, std::str::Utf8Error> {
        unsafe {CStr::from_ptr(GetClipboardText()).to_str()}
    }

    pub fn enable_event_waiting() {
        unsafe {EnableEventWaiting()}
    }

    pub fn disable_event_waiting() {
        unsafe {DisableEventWaiting()} 
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe{ CloseWindow() }
    }
}