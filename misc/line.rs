use eframe::egui;
use egui::{Color32, Id, Sense};
use egui_plot::{BoxElem, BoxPlot, BoxSpread, Line, Plot, PlotPoints};
fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    );
}

#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui
                .interact(ui.max_rect(), Id::new("window-drag"), Sense::drag())
                .dragged()
            {
                frame.drag_window();
            }

            let mylinedata: PlotPoints = vec![[0.0, 0.0], [10.0, 12.0], [20.0, 30.0]].into();

            let line = Line::new(mylinedata);
            Plot::new("my line plot")
                .view_aspect(2.0)
                .show(ui, |lineplots| lineplots.line(line));
        });
    }
}
