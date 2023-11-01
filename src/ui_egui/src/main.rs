#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod state;
use backend_gstreamer::GstreamerMediaManager;
use eframe::egui;
use egui::{load::SizedTexture, Image, ColorImage, Color32};
use state::State;

fn main() -> Result<(), eframe::Error> {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1000.0, 800.0)),
        ..Default::default()
    };
    eframe::run_native(
        "UnbreakEditApp",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::new(UnbreakEditApp::new())
        }),
    )
}

struct UnbreakEditApp {
    state: State,
}

impl UnbreakEditApp {
    pub fn new() -> Self {
        let media_manager = GstreamerMediaManager::new().unwrap();

        /* create a variable that points to the cargo manifest directory */
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let file = std::path::PathBuf::from(manifest_dir)
                    .parent()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .join("media/test.mp4")
                    .canonicalize()
                    .unwrap();
                println!("file [{}]: {}", file.exists(), file.display());

        let content = media_manager.create_media_container(&url::Url::from_file_path(file).unwrap(), false).unwrap();
        Self {
            state: State { media_manager, content }
        }
    }
}

impl eframe::App for UnbreakEditApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        ctx.request_repaint();
        let image_pixels = self.state.content.frame_image();
        let data = image_pixels.lock().unwrap();
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            
            if let Some(data) =  data.as_ref() {

                let (width, height) = self.state.content.size();
                
                let texture = ui.ctx().load_texture(
                    "video",
                    video_frame_to_image(width, height, data),
                    Default::default()
                    );
    
                let image = Image::new(SizedTexture::new(texture.id(), texture.size_vec2()));
                ui.add(image);

            }

            let paused = self.state.content.paused();
            if ui.button(if paused {"Play"} else {"Pause"}).clicked() {
              self.state
                .content
                .set_paused(!paused);
            }

            
        });
    }
}

fn video_frame_to_image(width: u32, height: u32, data: &[u8]) -> ColorImage {
    let size = [width as usize, height as usize];
    let pixel_size_bytes = 4;
    let pixels: Vec<_> = data.chunks_exact(pixel_size_bytes)
                .map(|p| Color32::from_rgb(p[0], p[1], p[2])).collect();
            println!("pixels: {}", pixels.len());
    ColorImage { size, pixels }
}