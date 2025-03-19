use core_graphics::display::CGDisplay;

use super::ScreenSize;

pub fn display_size() -> ScreenSize {
    let main = CGDisplay::main();
    ScreenSize {
        x: main.pixels_wide(),
        y: main.pixels_high(),
    }
}
