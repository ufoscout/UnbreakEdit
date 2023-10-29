use iced::{widget::{button, text, column, Component, component}, Sandbox, Element, Renderer};


// Define your application state.
pub struct MoviePlayerComponent {
    play_status: PlayStatus,
}

impl Default for MoviePlayerComponent {
    fn default() -> Self {
        MoviePlayerComponent {
            play_status: PlayStatus::Stop,
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

impl Sandbox for MoviePlayerComponent {
    type Message = MoviePlayerMessage;

    fn new() -> Self {
        MoviePlayerComponent {
            play_status: PlayStatus::Stop,
        }
    }

    fn update(&mut self, message: MoviePlayerMessage) {
        match message {
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
    }

    fn view(&self) -> Element<MoviePlayerMessage> {
        let text = text(format!("{:?}", self.play_status)).size(50);

        let button = button("Play/Stop")
            .on_press(MoviePlayerMessage::ButtonPressed);

        column![
            text,
            button,
        ].into()
    }

    fn title(&self) -> String {
        "MoviePlayerComponent".to_string()
    }
}

impl <Message> Component<Message, Renderer> for MoviePlayerComponent {

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
        let text = text(format!("{:?}", self.play_status)).size(50);

        let button = button("Play/Stop")
            .on_press(MoviePlayerMessage::ButtonPressed);

        column![
            text,
            button,
        ].into()
    }
}

impl<'a, Message> From<MoviePlayerComponent> for Element<'a, Message, Renderer>
    where
        Message: 'a,
    {
        fn from(my_component: MoviePlayerComponent) -> Self {
            component(my_component)
        }
    }