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

            let pointsss = vec![BoxElem::new(
                10.0,
                BoxSpread {
                    lower_whisker: 2.0,
                    quartile1: 2.0,
                    median: 3.0,
                    quartile3: 4.0,
                    upper_whisker: 4.0,
                },
            )];

            let abc = BoxPlot::new(pointsss).name("mybox").vertical();
            Plot::new("newboxing")
                .view_aspect(2.0)
                .show(ui, |build_ui| build_ui.box_plot(abc));
        });
    }
}
