use crate::auth_source::AuthSource;
use eframe::egui::{Align, Layout, RichText};
use eframe::{egui, epi};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct TemplateApp {
    tokens: Vec<AuthSource>,

    #[serde(skip)]
    show_add_dialog: bool,

    #[serde(skip)]
    new_token: AuthSource,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            tokens: vec![],
            show_add_dialog: false,
            new_token: AuthSource::default(),
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "2FA"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        let Self {
            tokens,
            show_add_dialog,
            new_token,
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        // egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        //     // The top panel is often a good place for a menu bar:
        //     egui::menu::bar(ui, |ui| {
        //         ui.menu_button("File", |ui| {
        //             if ui.button("Quit").clicked() {
        //                 frame.quit();
        //             }
        //         });
        //     });
        // });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            if ui.button("Add source").clicked() {
                *show_add_dialog = true;
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                if !tokens.is_empty() {
                    for (index, token) in tokens.clone().iter().enumerate() {
                        ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                            ui.label(RichText::new("TOK EN!").size(40.0));
                        });
                        ui.horizontal(|ui| {
                            // ui.label(RichText::new(token.issuer.as_str()).size(50.0));
                            ui.with_layout(egui::Layout::left_to_right(), |ui| {
                                ui.label(RichText::new(token.issuer.as_str()).size(32.0));
                                ui.label(RichText::new(token.name.as_str()).size(28.0))
                            });
                        });

                        if ui.button("Yeet").clicked() {
                            tokens.remove(index);
                        }
                    }
                } else {
                    ui.centered_and_justified(|ui| {
                        ui.label(
                            RichText::new("No tokens!\nClick \"Add source\" column to add one!")
                                .size(40.0),
                        );
                    });
                }
            });
        });

        if *show_add_dialog {
            egui::Window::new("Add").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Name");
                    ui.text_edit_singleline(&mut new_token.name);
                });

                ui.horizontal(|ui| {
                    ui.label("Issuer");
                    ui.text_edit_singleline(&mut new_token.issuer);
                });

                if ui.button("Add").clicked() {
                    tokens.push((*new_token).clone());
                    *new_token = AuthSource::default();
                    *show_add_dialog = false;
                }

                if ui.button("Cancel").clicked() {
                    *new_token = AuthSource::default();
                    *show_add_dialog = false;
                }
            });
        }
    }
}
