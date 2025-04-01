use eframe::CreationContext;
use eframe::epaint::text::FontInsert;
use eframe::epaint::text::InsertFontFamily;
use egui::Widget;
use egui::os::OperatingSystem;
use egui_modal_spinner::ModalSpinner;
use egui_notify::Anchor;
use egui_notify::Toasts;
use poll_promise::Promise;
use std::time::Duration;
use std::time::Instant;

use crate::llm;
use crate::model;
use crate::persistence;
use crate::shortcuts;

pub(crate) struct HanziApp {
    pub(crate) database_url: String,
    pub(crate) toasts: Toasts,
    pub(crate) spinner: ModalSpinner,
    pub(crate) is_macos: bool,
    pub(crate) input: String,
    pub(crate) llm_query: Option<Promise<Result<model::Phrase, llm::LLMError>>>,
    pub(crate) llm_query_start: Option<Instant>,
    pub(crate) phrase: Option<model::Phrase>,
    pub(crate) phrases: Vec<model::Phrase>,
    pub(crate) open_about: bool,
    pub(crate) open_settings: bool,
}

impl HanziApp {
    pub(crate) fn new(cc: &CreationContext<'_>, database_url: String) -> Self {
        cc.egui_ctx.add_font(FontInsert::new(
            "Han_Sans_CN_Light",
            egui::FontData::from_static(include_bytes!("../assets/Source Han Sans CN Light.otf")),
            vec![
                InsertFontFamily {
                    family: egui::FontFamily::Proportional,
                    priority: egui::epaint::text::FontPriority::Highest,
                },
                InsertFontFamily {
                    family: egui::FontFamily::Monospace,
                    priority: egui::epaint::text::FontPriority::Lowest,
                },
            ],
        ));
        Self {
            database_url,
            toasts: Toasts::default().with_anchor(Anchor::BottomRight),
            spinner: ModalSpinner::new()
                .spinner_size(60.)
                .spinner_color(egui::Color32::YELLOW),
            is_macos: cc.egui_ctx.os() == OperatingSystem::Mac,
            input: "学习汉语很有趣！".to_owned(),
            llm_query: None,
            llm_query_start: None,
            phrase: None,
            phrases: Vec::new(),
            open_about: false,
            open_settings: false,
        }
    }
}
