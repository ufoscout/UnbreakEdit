use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to create media manager. {0}")]
    MediaManagerCreationError(String),
    #[error("failed to get media capabilities. {0}")]
    MediaCapsError(String),
    #[error("failed to change media state. {0}")]
    MediaStateChangeError(String)
}