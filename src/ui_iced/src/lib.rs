use backend_gstreamer::GstreamerMediaManager;
use backend_gstreamer::media_container::MediaContainer;
use component::media_player::MediaPlayerComponent;
use component::movie_player::MoviePlayerComponent;
use iced::{Application, theme, executor, Length};
use iced::widget::container;

pub mod component;
pub mod page;
pub mod widget;

pub struct UnbreakEditMainApp {
    media_manager: GstreamerMediaManager,
    content: MediaContainer,
}

impl UnbreakEditMainApp {
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
            media_manager,
            content,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {}

impl Application for UnbreakEditMainApp {

    type Message = Message;
    type Theme = theme::Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self::new(),
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        "UnbreakEdit - Iced".to_owned()
    }

    fn update(&mut self, _message: Self::Message) -> iced::Command<Self::Message> {
        iced::Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        // let content = MoviePlayerComponent::default();

        let content = MediaPlayerComponent::new(&self.content);
        container(content)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
