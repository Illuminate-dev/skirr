use eframe::egui::text::LayoutJob;
use eframe::egui::{Align, Button, Color32, FontId, Id, Margin, Rounding, Stroke, Vec2, Visuals};
use eframe::Renderer;
use eframe::egui;
use poll_promise::Promise;
use crate::config::{Config, Script};
use crate::scrape::{search_with_term, Entry};
use crate::NAME;

mod settings;
use settings::Settings;


pub fn run_app() -> eframe::Result<()> {
    let viewport = egui::viewport::ViewportBuilder::default().with_inner_size(Vec2::new(800.0, 500.0));
    let options = eframe::NativeOptions {
        renderer: Renderer::Glow,
        default_theme: eframe::Theme::Light,
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        NAME,
        options,
        Box::new(|_ctx| Box::<App>::default())
    )
}

struct App {
    search_query: String,
    config: Config,
    selected_script: Option<Script>,
    result: Option<Promise<Vec<Entry>>>,
    toggle_settings: bool,
    settings: Settings,
}

impl Default for App {
    fn default() -> Self {
        let config = Config::default();
        let selected_script = config.get_default_script();
        Self {
            search_query: String::new(),
            settings: Settings::new(config.clone()),
            config, 
            selected_script,
            result: None,
            toggle_settings: false,
        }
    }
}

impl App {
    fn show_search_box(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, width: f32) {
        ui.visuals_mut().widgets.inactive.bg_stroke = Stroke::new(1.0, Color32::BLACK);
        ui.visuals_mut().widgets.inactive.rounding = Rounding::same(5.0);
        ui.visuals_mut().widgets.hovered.bg_stroke = Stroke::new(1.0, Color32::BLACK);
        ui.visuals_mut().widgets.hovered.rounding = Rounding::same(5.0);
        ui.visuals_mut().widgets.active.bg_stroke = Stroke::new(1.0, Color32::BLACK);
        ui.visuals_mut().widgets.active.rounding = Rounding::same(5.0);

        let height = 30.0;

        let search_id = Id::new("search_box");

        let mut layouter = |ui: &egui::Ui, string: &str, _wrap_width: f32| {
            let (string, color) = if !string.is_empty() || ui.memory(|m| m.focused()).is_some_and(|id| id == search_id) { (String::from(string), egui::Color32::DARK_GRAY) 
            } else { 
                (String::from("Search..."), egui::Color32::LIGHT_GRAY) 
            };

            let layout_job = LayoutJob::simple_singleline(string, FontId::default(), color);
            ui.fonts(|f| f.layout_job(layout_job))
        };

        let text_edit = egui::TextEdit::singleline(&mut self.search_query)
            .id(search_id)
            .vertical_align(Align::Center)
            .margin(Margin::same(10.0))
            .layouter(&mut layouter);
        let response = ui.add_sized(Vec2::new(width, height), text_edit);

        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            self.search(ctx);
        }


    }

    fn show_search_selector(&mut self, ui: &mut egui::Ui, width: f32) {
         egui::ComboBox::from_id_source("script_selector")
            .selected_text(self.selected_script.as_ref().map(|s| s.name.as_ref()).unwrap_or("None"))
            .width(width)
            .show_ui(ui, |ui| {
                for script in self.config.scripts.iter() {
                    ui.selectable_value(&mut self.selected_script, Some(script.clone()), &script.name);
                }
            });
    }

    fn show_settings_button(&mut self, ui: &mut egui::Ui, width: f32) {
        if ui.add(Button::new("⚙").min_size(Vec2::new(width, width))).clicked() {
            self.toggle_settings = !self.toggle_settings;
        }
    }

    fn show_search_bar(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {

        let searchbox_width = ui.available_width() / 3.0;
        let selector_width = ui.available_width() / 10.0;
        let settings_width = 20.0;
        let horizontal_padding = (ui.available_width() - searchbox_width - selector_width) / 2.0;

        ui.horizontal(|ui| {
            ui.add_space(horizontal_padding);
            self.show_search_box(ctx, ui, searchbox_width);
            self.show_search_selector(ui, selector_width);
            ui.add_space(horizontal_padding - settings_width * 2.0);
            self.show_settings_button(ui, settings_width);
            ui.add_space(settings_width);
        });
    }

    fn show_results(&mut self, ui: &mut egui::Ui) {

        // ui.horizontal_wrapped(|ui| {
        //     for res in &self.search_results {
        //         ui.label(res);
        //     }
        // });
        

        if let Some(promise) = &self.result {
            if let Some(entries) = promise.ready() {
                // match res {
                //     Ok(entry) => {
                //         ui.label(entry.main_text());
                //     }
                //     Err(e) => {
                //         ui.colored_label(ui.visuals().error_fg_color, if e.is_empty() { "Error" } else { e }); }
                // }
                for entry in entries {
                    entry.display(ui);
                }
            } else {
                ui.spinner();
            }
        }
    }

    fn search(&mut self, ctx: &egui::Context) {

        let ctx = ctx.clone();
        let script = if let Some(x) = self.selected_script.clone() { x.path } else { return; };
        let search_query = self.search_query.clone();

        let promise = Promise::spawn_thread("lua_search", move || {
            ctx.request_repaint();
            search_with_term(&script, &search_query)
        });

        self.result = Some(promise);

        // let results = crate::scrape::search_with_term("scripts/quotes.lua", &self.search_query);
        //
        // self.search_results = results.into_iter().map(|e| e.into_iter().find(|(k, _)| k == "main_text").unwrap().1).collect()
    }
}


impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // ctx.set_debug_on_hover(true);
        

        let visuals = Visuals {
            menu_rounding: Rounding::same(10.0),
            ..Visuals::light()
        };

        ctx.set_visuals(visuals);

        egui::TopBottomPanel::top("search_bar").show_separator_line(false).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                // show search bar
                ui.add_space(5.0);
                self.show_search_bar(ctx, ui);
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {self.show_results(ui);});
        });

        if self.toggle_settings {
            self.settings.show_settings(ctx, &mut self.config);
        }
    }
}

