use eframe::Renderer;
use eframe::egui;
use crate::NAME;

pub fn run_app() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        renderer: Renderer::Wgpu,
        ..Default::default()
    };

    eframe::run_native(
        NAME,
        options,
        Box::new(|_ctx| Box::<App>::default())
    )
}

#[derive(Default)]
struct App {}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello, world!");
        });
    }
}
