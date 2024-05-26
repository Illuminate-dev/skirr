use eframe::egui::text::LayoutJob;
use eframe::egui::Align;
use eframe::egui::Color32;
use eframe::egui::FontId;
use eframe::egui::Id;
use eframe::egui::Margin;
use eframe::egui::Rounding;
use eframe::egui::Stroke;
use eframe::egui::Vec2;
use eframe::egui::Visuals;
use eframe::Renderer;
use eframe::egui;
use crate::NAME;

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

#[derive(Default)]
struct App {
    search_query: String,
}

impl App {
    fn show_search_bar(&mut self, ui: &mut egui::Ui) {
        ui.visuals_mut().widgets.inactive.bg_stroke = Stroke::new(1.0, Color32::BLACK);
        ui.visuals_mut().widgets.inactive.rounding = Rounding::same(5.0);
        ui.visuals_mut().widgets.hovered.bg_stroke = Stroke::new(1.0, Color32::BLACK);
        ui.visuals_mut().widgets.hovered.rounding = Rounding::same(5.0);
        ui.visuals_mut().widgets.active.bg_stroke = Stroke::new(1.0, Color32::BLACK);
        ui.visuals_mut().widgets.active.rounding = Rounding::same(5.0);

        let width = ui.available_width() / 3.0;
        let height = 30.0;

        let search_id = Id::new("search_box");

        let mut layouter = |ui: &egui::Ui, string: &str, _wrap_width: f32| {
            let (string, color) = if !string.is_empty() || ui.memory(|m| m.focused()).is_some_and(|id| id == search_id) { 
                (String::from(string), egui::Color32::DARK_GRAY) 
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
        ui.add_sized(Vec2::new(width, height), text_edit);
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
                self.show_search_bar(ui);
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("Hello, world! You searched {}!", self.search_query));
        });
    }
}
