use core_graphics::display::CGDisplay;

use super::ScreenSize;
use super::ScreenSizeError;

pub fn display_size() -> Result<ScreenSize, ScreenSizeError> {
    let main = CGDisplay::main();
    Ok(ScreenSize {
        x: main.pixels_wide(),
        y: main.pixels_high(),
    })
}
