pub(crate) fn save(is_macos: bool) -> egui::KeyboardShortcut {
    egui::KeyboardShortcut::new(modifiers(is_macos), egui::Key::S)
}

pub(crate) fn find(is_macos: bool) -> egui::KeyboardShortcut {
    egui::KeyboardShortcut::new(modifiers(is_macos), egui::Key::F)
}

pub(crate) fn settings(is_macos: bool) -> egui::KeyboardShortcut {
    egui::KeyboardShortcut::new(modifiers(is_macos), egui::Key::Comma)
}

fn modifiers(is_macos: bool) -> egui::Modifiers {
    if is_macos {
        egui::Modifiers::MAC_CMD
    } else {
        egui::Modifiers::CTRL
    }
}
