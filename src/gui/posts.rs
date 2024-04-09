use iced::{
    widget::{row, Container},
    Command, Element,
};

#[derive(Debug, Clone)]
pub struct Posts {}

#[derive(Debug, Clone)]
pub enum Message {}

impl Posts {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, _message: Message) -> Command<Message> {
        Command::none()
    }

    pub fn view(&self) -> Element<Message> {
        Container::new(row!()).into()
    }
}

impl Default for Posts {
    fn default() -> Self {
        Self::new()
    }
}
