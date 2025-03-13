use eframe::CreationContext;
use eframe::epaint::text::FontInsert;
use eframe::epaint::text::InsertFontFamily;
use egui::Widget;
use egui_notify::Toasts;
use std::time::Duration;

use crate::shortcuts;

pub(crate) struct HanziApp {
    input: String,
    pinyin: String,
    translation: String,
    toasts: Toasts,
    is_macos: bool,
}

impl HanziApp {
    pub(crate) fn new(cc: &CreationContext<'_>) -> Self {
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
            input: "学习汉语很有趣!".to_owned(),
            pinyin: "Xuéxí hànyǔ hěn yǒuqù!".to_owned(),
            translation: "Learning Chinese is fun!".to_owned(),
            toasts: Toasts::default(),
            is_macos: std::env::consts::OS == "macos",
        }
    }
}

impl eframe::App for HanziApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.with_layout(
                    egui::Layout::left_to_right(egui::Align::TOP).with_main_justify(true),
                    |ui| {
                        egui::TextEdit::singleline(&mut self.input)
                            .horizontal_align(egui::Align::Center)
                            .text_color(egui::Color32::YELLOW)
                            .margin(egui::Margin::same(16))
                            .font(egui::FontId::new(54., egui::FontFamily::Proportional))
                            .ui(ui)
                    },
                );
                egui::Frame::new().inner_margin(14.).show(ui, |ui| {
                    ui.columns_const(|[col_1, col_2]| {
                        col_1.vertical(|ui| ui.label(egui::RichText::new(&self.pinyin).size(28.)));
                        col_2.vertical(|ui| ui.label(egui::RichText::new(&self.translation).size(28.)));
                    })
                });
            });
        });

        if ctx.input_mut(|i| i.consume_shortcut(&shortcuts::save(self.is_macos))) {
            self.toasts
                .info("This is where the phrase with pinyin and translation will be saved")
                .duration(Some(Duration::from_secs(5)))
                .show_progress_bar(true);
        }
        if ctx.input_mut(|i| i.consume_shortcut(&shortcuts::find(self.is_macos))) {
            self.toasts
                .info("This is where the search for phrases will open")
                .duration(Some(Duration::from_secs(5)))
                .show_progress_bar(true);
        }
        if ctx.input_mut(|i| i.consume_shortcut(&shortcuts::settings(self.is_macos))) {
            self.toasts
                .info("This is where the settings will open")
                .duration(Some(Duration::from_secs(5)))
                .show_progress_bar(true);
        }

        self.toasts.show(ctx);
    }
}
