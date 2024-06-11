use eframe::egui::{self, ViewportBuilder};
use egui::{ColorImage, TextureHandle, TextureOptions, ImageButton};

struct MyApp {
    shutdown_texture: TextureHandle,
    reboot_texture: TextureHandle,
    logout_texture: TextureHandle,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load your images here and get their TextureId
        let shutdown_texture = load_texture(&cc.egui_ctx, include_bytes!("./shutdown.png"));
        let reboot_texture = load_texture(&cc.egui_ctx, include_bytes!("./reboot.png"));
        let logout_texture = load_texture(&cc.egui_ctx, include_bytes!("./logout.png"));

        Self {
            shutdown_texture,
            reboot_texture,
            logout_texture,
        }
    }

    fn shutdown(&self) {
        std::process::Command::new("shutdown").arg("-h").arg("now").spawn().unwrap();
    }

    fn reboot(&self) {
        std::process::Command::new("reboot").spawn().unwrap();
    }
    fn logout(&self) {
        std::process::Command::new("logout").spawn().unwrap();
    }
}

fn load_texture(ctx: &egui::Context, bytes: &[u8]) -> TextureHandle {
    let image = image::load_from_memory(bytes).expect("Failed to load image");
    let image = image.resize(image.width() / 2, image.height() / 2, image::imageops::FilterType::Lanczos3);
    let size = [image.width() as usize, image.height() as usize];
    let pixels = image.into_rgba8().into_raw();

    // Create ColorImage from raw pixels
    let color_image = ColorImage::from_rgba_unmultiplied(size, &pixels);

    // Load the texture with options
    ctx.load_texture(
        "image",
        color_image,
        TextureOptions::LINEAR, // or TextureOptions::NEAREST
    )
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.horizontal_centered(|ui| {
                    ui.add_space(ui.available_width() / 2.0 - ((64.0)*1.5)-30.0);
                    if ui.add(ImageButton::new(&self.shutdown_texture).frame(false)).clicked() {
                        self.shutdown();
                    }
                    ui.add_space(30.0);
                    if ui.add(ImageButton::new(&self.reboot_texture).frame(false)).clicked() {
                        self.reboot();
                    }
                    ui.add_space(30.0);
                    if ui.add(ImageButton::new(&self.logout_texture).frame(false)).clicked() {
                        self.logout();
                    }
                });
            });
        });
    }
}

fn main() {
    let options = eframe::NativeOptions{
        viewport: ViewportBuilder::default().with_inner_size([300.0, 100.0]),
        centered: true,
        ..Default::default()
    };
    let _ = eframe::run_native(
        "pm",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    );
}
