use std::num::TryFromIntError;
use thiserror::Error;

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

#[derive(Debug, Error)]
pub enum ScreenSizeError {
    #[error("Screen size is invalid: {0}")]
    InvalidSize(#[source] TryFromIntError), //windows or linux only
    #[error("No screen found")]
    NoScreen, //linux only
}

/// code shamelessly stolen from https://github.com/emmabritton/screen_size
#[inline(always)]
pub fn get_primary_screen_size() -> Result<ScreenSize, ScreenSizeError> {
    display_size_impl()
}
