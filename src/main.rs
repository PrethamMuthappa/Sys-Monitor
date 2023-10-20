use eframe::{egui, HardwareAcceleration, Theme};
use egui::{Id, Sense, Vec2, Layout, RichText, Color32};


fn main() {
    let nativeoption=eframe::NativeOptions {
        always_on_top: false,
        maximized: false,
        decorated: false,
        fullscreen: false,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_pos: None,
        initial_window_size: Option::from(Vec2::new(800f32,500f32)),
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
        hardware_acceleration: HardwareAcceleration::Required,
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
struct Res {}

impl Res {
    fn new (_cc:&eframe::CreationContext<'_>) -> Self {
   
   Self::default()    

}


}

impl eframe::App for Res {

fn update(&mut self, ctx:&egui::Context, frame:&mut eframe::Frame) {

egui::CentralPanel::default().show(ctx,|ui| {

egui::ScrollArea::vertical().show(ui,|ui| { 

if ui.interact(ui.max_rect(), Id::new("window-drag"),Sense::drag()).dragged(){ 
          frame.drag_window();
 }

    ui.heading(RichText::new("resource monitor").color(Color32::RED));


    ui.separator();



 });
 }); 



}

}
