use std::{path::PathBuf, time::Duration};

use backend_gstreamer::GstreamerMediaManager;

#[test]
fn test_should_open_mp4_file() {
    let media_manager = GstreamerMediaManager::new().unwrap();
    let file = get_media_test_dir().join("test_001.mp4");
    assert!(file.exists());

    let media_container = media_manager.create_media_container(&url::Url::from_file_path(file.clone()).unwrap(), false).unwrap();
    assert!(media_container.paused());
    // assert_eq!((720, 480), media_container.size());
    assert_eq!((853, 480), media_container.size());
    assert_eq!(Duration::from_millis(19051), media_container.duration());
    assert_eq!(24000, *media_container.framerate().numer());
    assert_eq!(1001, *media_container.framerate().denom());
}

#[test]
fn test_should_open_mp4_file_and_send_frames() {
    let media_manager = GstreamerMediaManager::new().unwrap();
    let file = get_media_test_dir().join("test_001.mp4");
    assert!(file.exists());

    let mut media_container = media_manager.create_media_container(&url::Url::from_file_path(file.clone()).unwrap(), false).unwrap();
    assert!(media_container.paused());
    
    let receiver = media_container.frame_receiver().clone();
    
    // The media container should be paused, so no frames should be sent
    assert!(receiver.try_recv().is_err());

    media_container.set_paused(false);
    assert!(!media_container.paused());

    let frame = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
    assert!(!frame.is_empty());
}

#[test]
fn media_test_dir_should_exist() {
    assert!(get_media_test_dir().exists());
}


fn get_media_test_dir() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    std::path::PathBuf::from(manifest_dir)
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .join("media/tests")
                .canonicalize()
                .unwrap()
}