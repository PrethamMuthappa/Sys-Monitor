use std::thread::sleep;
use eframe::{egui, HardwareAcceleration, Theme};
use eframe::emath::Align;
use egui::{Id, Sense, Vec2, RichText, Color32, Layout, FontId};



fn main() {
    let nativeoption=eframe::NativeOptions {
        always_on_top: false,
        maximized: false,
        decorated: true,
        fullscreen: false,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_pos: None,
        initial_window_size: Option::from(Vec2::new(850f32,650f32)),
        min_window_size: None,
        max_window_size: None,
        resizable: true,
        transparent: false,
        mouse_passthrough: false,
        active: false,
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        hardware_acceleration: HardwareAcceleration::Off,
        renderer: Default::default(),
        follow_system_theme: false,
        default_theme: Theme::Dark,
        run_and_return: false,
        event_loop_builder: None,
        window_builder: None,
        shader_version: None,
        centered: false,
        app_id: None,
        persist_window: false,
    };
    eframe::run_native(
        "resource-monitor",
        nativeoption,Box::new(|cc| Box::new(Res::new(cc))),
    )
        .expect("error")
}

#[derive(Default)]
struct Res {
    name:String,
}

impl Res {
    fn new (_cc:&eframe::CreationContext<'_>) -> Self {

   Self::default()

}

    fn data(&self) {}


}

impl eframe::App for Res {

fn update(&mut self, ctx:&egui::Context, frame:&mut eframe::Frame) {

egui::CentralPanel::default().show(ctx,|ui| {
   

egui::ScrollArea::vertical().show(ui,|ui| { 

if ui.interact(ui.max_rect(), Id::new("window-drag"),Sense::drag()).dragged(){ 
          frame.drag_window();
 }

    ui.with_layout(Layout::top_down(Align::Center), |ui| {
        ui.heading(RichText::new("Resource monitor").color(Color32::RED).font(FontId::monospace(28.5)));
    });


    ui.separator();

    ui.label(RichText::new("CPU").color(Color32::GREEN).font(FontId::monospace(20.0)));

    if ui.button("click").clicked() {
    self.name="pretham".to_string();
    }


    if ui.button("clear").clicked() {
        self.name.clear();
    }

    ui.label(&self.name);

 });
 }); 



}

}
