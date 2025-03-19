#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "linux")]
use linux::display_size as display_size_impl;
#[cfg(target_os = "macos")]
use macos::display_size as display_size_impl;

pub(crate) struct ScreenSize {
    pub(crate) x: u64,
    pub(crate) y: u64,
}

#[inline(always)]
pub fn get_primary_screen_size() -> ScreenSize {
    display_size_impl()
}
