use backend_gstreamer::media_container::MediaContainer;
use iced::{widget::{button, text, column, Component, component, Image, image::Handle}, Element, Renderer};


// Define your application state.
pub struct MediaPlayerComponent<'a> {
    play_status: PlayStatus,
    media: &'a MediaContainer,
}

impl <'a> MediaPlayerComponent<'a> {
    pub fn new(media: &'a MediaContainer) -> Self {
        MediaPlayerComponent {
            play_status: PlayStatus::Stop,
            media
        }
    }
}

#[derive(Debug, Clone)]
enum PlayStatus {
    Play,
    Stop
}

// Define messages that can be sent to your application.
#[derive(Debug, Clone)]
pub enum MoviePlayerMessage {
    ButtonPressed,
}

impl <'a, Message> Component<Message, Renderer> for MediaPlayerComponent<'a> {

    type State = ();
    type Event = MoviePlayerMessage;

    fn update(
        &mut self,
        _state: &mut Self::State,
        event: Self::Event,
    ) -> Option<Message> {
        match event {
            MoviePlayerMessage::ButtonPressed => {
                match self.play_status {
                    PlayStatus::Play => {
                        self.play_status = PlayStatus::Stop;
                    },
                    PlayStatus::Stop => {
                        self.play_status = PlayStatus::Play;
                    }
                }
            }
        }       
        None
    }

    fn view(&self, _state: &Self::State) -> iced::advanced::graphics::core::Element<'_, Self::Event, Renderer> {
        let image_pixels = self.media.frame_image();
        let image_pixels = image_pixels.lock().unwrap();
        let (width, height) = self.media.size();

        let image = Image::new(Handle::from_pixels(
            width as _,
            height as _,
            (*image_pixels).clone().unwrap_or_default(),
        ));
        let text = text(format!("{:?}", self.play_status)).size(50);

        let button = button("Play/Stop")
            .on_press(MoviePlayerMessage::ButtonPressed);

        column![
            image,
            text,
            button,
        ].into()
    }
}

impl<'a, Message> From<MediaPlayerComponent<'a>> for Element<'a, Message, Renderer>
    where
        Message: 'a,
    {
        fn from(my_component: MediaPlayerComponent<'a>) -> Self {
            component(my_component)
        }
    }

