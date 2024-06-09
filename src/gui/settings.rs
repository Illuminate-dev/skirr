use eframe::egui::{self, RichText, Ui, Vec2};

use crate::config::{Config, Script};

#[derive(Debug, Clone, Copy, PartialEq)]
enum SettingsTab {
    General,
    Scripts,
}

impl SettingsTab {
    const ALL: [Self; 2] = [SettingsTab::General, SettingsTab::Scripts];

    fn to_string(self) -> &'static str {
        match self {
            Self::General => "General",
            Self::Scripts => "Scripts", }
    }
}

pub struct Settings {
    tab: SettingsTab,
    temp_config: Config,
}

impl Settings {
    pub fn new(cfg: Config) -> Self {
        Self {
            temp_config: cfg,
            tab: SettingsTab::General,
        }
    }
}

impl Settings { // todo: assign sizes dynamically
    pub fn show_settings(&mut self, ctx: &egui::Context, config: &mut Config) {

        egui::Window::new("Settings")
            // .max_height(ctx.input(|i| i.viewport().outer_rect.map(|x| x.height() * 0.5).unwrap_or(200.0)))
            .max_size(Vec2::new(600.0, ctx.input(|i| i.viewport().outer_rect.map(|x| x.height() * 0.5).unwrap_or(200.0))))
            .show(ctx, |ui| {
            egui::SidePanel::left("settings_panel").resizable(false).max_width(55.0).show_inside(ui, |ui| {
                for tab in SettingsTab::ALL.iter() {
                    ui.selectable_value(&mut self.tab, *tab, tab.to_string());
                }
            });

            ui.horizontal(|ui| {
                // ui.set_width(ui.available_width());
                // ui.set_height(ui.available_height());
                ui.add_space(5.0);
                ui.vertical(
                    |ui| {
                        match self.tab {
                            SettingsTab::General => self.show_general(ui),
                            SettingsTab::Scripts => self.show_scripts(ui),
                        }
                        if ui.button("Save").on_hover_text("Save changes").clicked() {
                            // TODO: validate config before saving
                            *config = self.temp_config.clone();
                            config.save();
                        }
                    }
                );

                ui.add_space(5.0);
            });
        });
    } 

    fn show_general(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| ui.label(RichText::new("General").heading()));
    }
    
    fn show_scripts(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| ui.label(RichText::new("Scripts").heading()));

        for script in self.temp_config.scripts.iter_mut() {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut script.name);
                ui.text_edit_singleline(&mut script.path);
            });
        }

        if ui.button("Add script").clicked() {
            self.temp_config.scripts.push(Script::new("", ""));
        }

    }
}

