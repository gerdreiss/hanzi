use std::convert::TryInto;
use std::ptr::null;
use x11::xlib;

use super::ScreenSize;
use super::ScreenSizeError;

pub fn display_size() -> Result<ScreenSize, ScreenSizeError> {
    unsafe {
        let display = xlib::XOpenDisplay(null());
        if display.is_null() {
            return Err(ScreenSizeError::NoScreen);
        }
        let screen_ptr = xlib::XDefaultScreenOfDisplay(display);
        if screen_ptr.is_null() {
            return Err(ScreenSizeError::NoScreen);
        }
        let screen = *screen_ptr;
        Ok(ScreenSize {
            x: screen.width.try_into().map_err(ScreenSizeError::InvalidSize)?,
            y: screen.height.try_into().map_err(ScreenSizeError::InvalidSize)?,
        })
    }
}
