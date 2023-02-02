#[allow(unused)]

pub mod window;
use bitflags::bitflags;
pub use raylibrs_sys::math as math;

bitflags! {
    #[repr(C)]
    struct ConfigFlags: u32 {
        const VSYNC_HINT        = 0x00000040;
        const FULLSCREEN_MODE   = 0x00000002;
        const WINDOW_RESIZABLE  = 0x00000004;
        const WINDOW_UNDECORATED= 0x00000008;
        const WINDOW_HIDDEN     = 0x00000080;
        const WINDOW_MINIMIZED  = 0x00000200;
        const WINDOW_MAXIMIZED  = 0x00000400;
        const WINDOW_UNFOCUSED  = 0x00000800;
        const WINDOW_TOPMOST    = 0x00001000;
        const WINDOW_ALWAYS_RUN = 0x00000100;
        const WINDOW_TRANSPARENT= 0x00000010;
        const WINDOW_HIGHDPI    = 0x00002000;
        const WINDOW_MOUSE_PASSTHROUGH  = 0x00004000;
        const MSAA_4X_HINT      = 0x00000020;
        const INTERLACED_HINT   = 0x00010000;
    }
}