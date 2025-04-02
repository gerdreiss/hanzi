use egui::Widget;
use std::time::Duration;

use crate::app;
use crate::shortcuts;

impl eframe::App for app::HanziApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // CREATE UI
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.with_layout(
                    egui::Layout::left_to_right(egui::Align::TOP).with_main_justify(true),
                    |ui| {
                        egui::TextEdit::singleline(&mut self.phrase_input)
                            .id(egui::Id::new("hanzi_editor"))
                            .horizontal_align(egui::Align::Center)
                            .text_color(egui::Color32::YELLOW)
                            .margin(egui::Margin::same(16))
                            .font(egui::FontId::new(64., egui::FontFamily::Proportional))
                            .ui(ui)
                    },
                );
                egui::Frame::NONE.inner_margin(18.).show(ui, |ui| {
                    if self.phrase.is_some() {
                        ui.columns_const(|[col_1, col_2]| {
                            col_1.horizontal(|ui| {
                                if self.edit_result {
                                    ui.with_layout(
                                        egui::Layout::left_to_right(egui::Align::LEFT).with_main_justify(true),
                                        |ui| {
                                            egui::TextEdit::singleline(&mut self.pinyin_input)
                                                .id(egui::Id::new("pinyin_editor"))
                                                .font(egui::FontId::new(28., egui::FontFamily::Proportional))
                                                .ui(ui)
                                        },
                                    );
                                } else if let Some(p) = &self.phrase {
                                    ui.label(egui::RichText::new(p.pinyin.clone()).size(28.));
                                } else {
                                    ui.label("");
                                }
                            });
                            col_2.horizontal(|ui| {
                                if self.edit_result {
                                    ui.with_layout(
                                        egui::Layout::left_to_right(egui::Align::LEFT).with_main_justify(true),
                                        |ui| {
                                            egui::TextEdit::singleline(&mut self.translation_input)
                                                .id(egui::Id::new("translation_editor"))
                                                .font(egui::FontId::new(28., egui::FontFamily::Proportional))
                                                .ui(ui)
                                        },
                                    );
                                } else if let Some(p) = &self.phrase {
                                    ui.label(egui::RichText::new(p.translation.clone()).size(28.));
                                } else {
                                    ui.label("");
                                }
                            });
                        });
                    } else if !self.phrases.is_empty() {
                        self.phrases.iter().for_each(|phrase| {
                            ui.columns_const(|[col_1, col_2, col_3]| {
                                col_3
                                    .vertical(|ui| ui.label(egui::RichText::new(phrase.translation.clone()).size(28.)));
                                col_2.vertical(|ui| ui.label(egui::RichText::new(phrase.pinyin.clone()).size(28.)));
                                col_1.vertical_centered_justified(|ui| {
                                    ui.label(
                                        egui::RichText::new(phrase.original.clone())
                                            .color(egui::Color32::YELLOW)
                                            .size(44.),
                                    )
                                });
                            });
                        });
                    }
                });
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    egui::Hyperlink::from_label_and_url("egui", "https://github.com/emilk/egui").ui(ui);
                    egui::Label::new("Powered by").ui(ui);
                    ui.separator();
                });
            });
        });

        // HANDLE EVENTS
        if ctx.input_mut(|i| i.consume_shortcut(&shortcuts::edit(self.is_macos))) {
            self.edit();
        }
        if ctx.input_mut(|i| i.consume_shortcut(&shortcuts::learn(self.is_macos))) {
            self.learn();
        }
        if ctx.input_mut(|i| i.consume_shortcut(&shortcuts::save(self.is_macos))) {
            self.save_phrase();
        }
        if ctx.input_mut(|i| i.consume_shortcut(&shortcuts::find(self.is_macos))) {
            self.read_phrases();
        }
        if ctx.input_mut(|i| i.consume_shortcut(&shortcuts::settings(self.is_macos))) {
            self.toasts
                .info("This is where the settings will open")
                .duration(Some(Duration::from_secs(5)))
                .show_progress_bar(true);
            self.open_settings = !self.open_settings;
        }
        if ctx.input_mut(|i| i.consume_shortcut(&shortcuts::about(self.is_macos))) {
            self.open_about = !self.open_about;
        }
        if ctx.input(|i| i.key_pressed(egui::Key::F1)) {
            self.toasts
                .info("This is where the help will be displayed")
                .duration(Some(Duration::from_secs(5)))
                .show_progress_bar(true);
            self.open_help = !self.open_help;
        }
        if ctx.input(|i| i.key_pressed(egui::Key::Enter)) && self.llm_query.is_none() {
            self.query_llm();
        }
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.handle_escape();
        }

        // HANDLE LLM QUERIES
        if let Some(query) = self.llm_query.take() {
            match query.try_take() {
                Ok(Ok(response)) => {
                    self.llm_query = None;
                    self.llm_query_start = None;
                    self.spinner.close();
                    self.phrase_input = response.original.clone();
                    self.phrase = Some(response);
                }
                Ok(Err(err)) => {
                    log::error!("Error occurred when querying LLM: {} caused by {}", err, err.cause());
                    self.llm_query = None;
                    self.llm_query_start = None;
                    self.spinner.close();
                    self.toasts
                        .error(format!("Querying LLM failed: {}", err.cause()))
                        .duration(Some(Duration::from_secs(5)))
                        .show_progress_bar(true);
                }
                Err(promise) => {
                    if let Some(start) = self.llm_query_start {
                        if start.elapsed().as_secs() > 60 {
                            promise.abort();
                            self.llm_query = None;
                            self.llm_query_start = None;
                            self.spinner.close();
                            self.toasts
                                .error("LLM query timed out")
                                .duration(Some(Duration::from_secs(5)))
                                .show_progress_bar(true);
                        } else {
                            self.llm_query = Some(promise)
                        }
                    } else {
                        self.llm_query = Some(promise)
                    }
                }
            }
        }

        if self.open_about {
            egui::Window::new("About").auto_sized().show(ctx, |ui| {
                egui::Frame::NONE.inner_margin(18.).show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new("Hanzi").size(20.).color(egui::Color32::YELLOW));
                        ui.label(egui::RichText::new("A little helper for Chinese learners").size(20.));
                        ui.label(egui::RichText::new("Copyright (c) 2025, Gerd Reiss").size(20.));
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Written In").size(20.));
                            egui::Hyperlink::from_label_and_url(
                                egui::RichText::new("Rust").size(20.),
                                "https://www.rust-lang.org",
                            )
                            .ui(ui);
                        });
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Powered by").size(20.));
                            egui::Hyperlink::from_label_and_url(
                                egui::RichText::new("Egui").size(20.),
                                "https://github.com/emilk/egui",
                            )
                            .ui(ui);
                            ui.label(egui::RichText::new("and").size(20.));
                            egui::Hyperlink::from_label_and_url(
                                egui::RichText::new("Ollama").size(20.),
                                "https://ollama.com/",
                            )
                            .ui(ui);
                        });
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Hosted on").size(20.));
                            egui::Hyperlink::from_label_and_url(
                                egui::RichText::new("Github").size(20.),
                                "https://github.com/gerdreiss/hanzi",
                            )
                            .ui(ui);
                        });
                    });
                });
            });
        }

        self.spinner.update_with_content(ctx, |ui| {
            ui.label("Querying LLM...");
        });

        self.toasts.show(ctx);
    }
}
