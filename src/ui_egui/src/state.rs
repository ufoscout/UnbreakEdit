use backend_gstreamer::{GstreamerMediaManager, media_container::MediaContainer};
use egui::TextureHandle;


pub struct State {
    pub media_manager: GstreamerMediaManager,
    pub content: Vec<MediaState>,
}

pub struct MediaState {
    pub last_texture: Option<TextureHandle>,
    pub media_container: MediaContainer,
}