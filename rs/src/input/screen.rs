#![cfg(windows)]

use crate::globals;

use std::io::Error;
use std::mem::MaybeUninit;

use winapi::um::winuser::{GetDesktopWindow, GetWindowRect};

use winapi::um::wingdi::{GetPixel, CLR_INVALID};

/// Gets the primary screen's size as `(width, height)`.
///
/// # References
///
/// https://docs.microsoft.com/en-us/windows/desktop/api/winuser/nf-winuser-getdesktopwindow
pub fn size() -> Result<(usize, usize), Error> {
    unsafe {
        let mut desktop = MaybeUninit::uninit();
        let handle = GetDesktopWindow();
        if GetWindowRect(handle, desktop.as_mut_ptr()) == 0 {
            Err(Error::last_os_error())
        } else {
            let desktop = desktop.assume_init();
            Ok((desktop.right as usize, desktop.bottom as usize))
        }
    }
}

/// Gets an on-screen color as `(r, g, b)`.
///
/// # References
///
/// https://docs.microsoft.com/en-us/windows/desktop/api/wingdi/nf-wingdi-getpixel
pub fn color(x: usize, y: usize) -> Result<(u8, u8, u8), &'static str> {
    unsafe {
        if let Some(dc) = globals::get_desktop_dc() {
            let color = GetPixel(dc, x as i32, y as i32);
            if color != CLR_INVALID {
                Ok((
                    ((color >> 0) & 0xff) as u8,
                    ((color >> 2) & 0xff) as u8,
                    ((color >> 4) & 0xff) as u8,
                ))
            } else {
                Err("failed to get pixel")
            }
        } else {
            Err("failed to get desktop dc")
        }
    }
}