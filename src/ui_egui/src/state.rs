use backend_gstreamer::{GstreamerMediaManager, media_container::MediaContainer};


pub struct State {
    pub media_manager: GstreamerMediaManager,
    pub content: MediaContainer,
}