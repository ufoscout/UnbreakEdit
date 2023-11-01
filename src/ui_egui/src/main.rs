#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod state;
use backend_gstreamer::GstreamerMediaManager;
use eframe::egui;
use egui::{load::SizedTexture, Image, ColorImage, Color32};
use state::State;

use crate::state::MediaState;

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

                let mut content = vec![];

                content.push(MediaState {
                    last_texture: None,
                    media_container: media_manager.create_media_container(&url::Url::from_file_path(file.clone()).unwrap(), false).unwrap()
                });
        // content.push(media_manager.create_media_container(&url::Url::from_file_path(file).unwrap(), false).unwrap());
        Self {
            state: State { media_manager, content }
        }
    }
}

impl eframe::App for UnbreakEditApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // let thread_id = std::thread::current().id();
        // println!("thread_id {:?} - update", thread_id);

        ctx.request_repaint();
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            
            for content in &mut self.state.content {

                if let Ok(data) =  content.media_container.frame_receiver().try_recv() {
                    let (width, height) = content.media_container.size();
                    content.last_texture = Some(ui.ctx().load_texture(
                        "video",
                        video_frame_to_image(width, height, &data),
                        Default::default()
                        ));
                }

                if let Some(texture) = &content.last_texture {
                    let image = Image::new(SizedTexture::new(texture.id(), texture.size_vec2()));
                    ui.add(image);
                    
                        let paused = content.media_container.paused();
                        if ui.button(if paused {"Play"} else {"Pause"}).clicked() {
                            content.media_container
                            .set_paused(!paused);
                    }
                }

            }


            
        });
    }
}

fn video_frame_to_image(width: u32, height: u32, data: &[u8]) -> ColorImage {
    let size = [width as usize, height as usize];
    let pixel_size_bytes = 4;
    let pixels: Vec<_> = data.chunks_exact(pixel_size_bytes)
                .map(|p| Color32::from_rgba_premultiplied(p[0], p[1], p[2], p[3])).collect();
            // println!("pixels: {}", pixels.len());
    ColorImage { size, pixels }
}