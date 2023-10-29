use component::movie_player::MoviePlayerComponent;
use iced::{Application, theme, executor, Length};
use iced::widget::container;

pub mod component;
pub mod page;
pub mod widget;

pub struct UnbreakEditMainApp {

}

impl UnbreakEditMainApp {
    pub fn new() -> Self {
        Self {

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
        let content = MoviePlayerComponent::default();

        container(content)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
