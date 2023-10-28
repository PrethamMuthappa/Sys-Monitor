use eframe::emath::Align;
use eframe::{egui, HardwareAcceleration, Theme};
use egui::{Color32, FontId, Id, Layout, RichText, Sense, Vec2};
use std::collections::VecDeque;
use std::thread;
use std::time::Duration;
use systemstat::{saturating_sub_bytes, Platform, System};
fn main() {
    let nativeoption = eframe::NativeOptions {
        always_on_top: false,
        maximized: false,
        decorated: true,
        fullscreen: false,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_pos: None,
        initial_window_size: Option::from(Vec2::new(850f32, 650f32)),
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
        nativeoption,
        Box::new(|cc| Box::new(Res::new(cc))),
    )
    .expect("error")
}

#[derive(Default)]
struct Res {}

impl Res {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for Res {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                if ui
                    .interact(ui.max_rect(), Id::new("window-drag"), Sense::drag())
                    .dragged()
                {
                    frame.drag_window();
                }

                ui.with_layout(Layout::top_down(Align::Center), |ui| {
                    ui.heading(
                        RichText::new("Resource monitor")
                            .color(Color32::RED)
                            .font(FontId::monospace(28.5)),
                    );
                });

                ui.separator();

                ui.label(
                    RichText::new("CPU")
                        .color(Color32::GREEN)
                        .font(FontId::monospace(20.0)),
                );

                let sys = System::new();

                let mut live = VecDeque::new();

                match sys.cpu_load_aggregate() {
                    Ok(cpu) => {
                        thread::sleep(Duration::from_secs(1));
                        let cpu = cpu.done().unwrap();
                        live.push_front(cpu.user);
                    }
                    Err(error) => println!("{:?}", error),
                }

                ui.label(
                    RichText::new(format!("printing values from cpu load\t{:?}", live))
                        .font(FontId::monospace(18.1)),
                );

                ui.separator();

                ui.label(
                    RichText::new("MEMORY")
                        .color(Color32::GREEN)
                        .font(FontId::monospace(20.0)),
                );

                match sys.memory() {
                    Ok(mem) => {
                        ui.label(
                            RichText::new(format!(
                                "\nmemory {} used/{} ({} bytes)",
                                saturating_sub_bytes(mem.total, mem.free),
                                mem.total,
                                mem.total.as_u64(),
                            ))
                            .font(FontId {
                                size: 18.1,
                                family: egui::FontFamily::Monospace,
                            }),
                        );
                    }
                    Err(err) => println!("{:?}", err),
                }

                ui.separator();

                ui.label(
                    RichText::new("SWAP")
                        .color(Color32::GREEN)
                        .font(FontId::monospace(20.0)),
                );

                match sys.swap() {
                    Ok(swap) => {
                        ui.label(
                            RichText::new(format!(
                                "\nswap: {} used / {} ({} bytes)",
                                saturating_sub_bytes(swap.total, swap.free),
                                swap.total,
                                swap.total.as_u64(),
                            ))
                            .font(FontId {
                                size: 18.1,
                                family: egui::FontFamily::Monospace,
                            }),
                        );
                    }

                    Err(err) => println!("{:?}", err),
                }

                ui.separator();

                ui.label(
                    RichText::new("average load")
                        .color(Color32::GREEN)
                        .font(FontId::monospace(20.0)),
                );

                match sys.load_average() {
                    Ok(avg) => {
                        ui.label(
                            RichText::new(format!(
                                "\nload average {} {} {}",
                                avg.one, avg.five, avg.fifteen
                            ))
                            .font(FontId {
                                size: 18.1,
                                family: egui::FontFamily::Monospace,
                            }),
                        );
                    }
                    Err(err) => println!("{:?}", err),
                }

                ui.separator();

                ui.label(
                    RichText::new("UP-TIME")
                        .color(Color32::GREEN)
                        .font(FontId::monospace(20.0)),
                );

                match sys.uptime() {
                    Ok(up) => {
                        ui.label(RichText::new(format!("\nuptime : {:?}", up)).font(FontId {
                            size: 18.1,
                            family: egui::FontFamily::Monospace,
                        }));
                    }
                    Err(err) => println!("{:?}", err),
                }

                ui.separator();

                ui.label(
                    RichText::new("Boot-Time")
                        .color(Color32::GREEN)
                        .font(FontId::monospace(20.0)),
                );

                match sys.boot_time() {
                    Ok(boot) => {
                        ui.label(
                            RichText::new(format!("\nBoot time: {}", boot)).font(FontId {
                                size: 18.1,
                                family: egui::FontFamily::Monospace,
                            }),
                        );
                    }
                    Err(err) => println!("{:?}", err),
                }

                ui.separator();

                ui.label(
                    RichText::new("CPU-TEMP")
                        .color(Color32::GREEN)
                        .font(FontId::monospace(20.0)),
                );

                match sys.socket_stats() {
                    Ok(temp) => {
                        ui.label(
                            RichText::new(format!("\nsystem socket statistics: {:?}", temp)).font(
                                FontId {
                                    size: 18.1,
                                    family: egui::FontFamily::Monospace,
                                },
                            ),
                        );
                    }
                    Err(err) => println!("{:?}", err),
                }

                ui.separator();
            });
        });
    }
}
