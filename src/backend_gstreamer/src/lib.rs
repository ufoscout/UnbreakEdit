use backend_common::error::Error;

pub mod error;
pub mod media_container;

/// A media manager that uses Gstreamer to play media.
pub struct GstreamerMediaManager {

}

impl GstreamerMediaManager {

    /// Creates a new GstreamerMediaManager.
    pub fn new() -> Result<Self, Error> {
        gstreamer::init().map_err(|e| Error::MediaManagerCreationError(format!("{e:?}")))?;
        Ok(GstreamerMediaManager {})
    }

    /// Creates a new media container.
    pub fn create_media_container(&self, uri: &url::Url, live: bool) -> Result<media_container::MediaContainer, Error> {
        media_container::MediaContainer::new(uri, live)
    }
}