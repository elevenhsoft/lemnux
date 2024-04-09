use iced::{
    widget::{button, Container},
    Command, Element,
};

#[derive(Debug, Clone)]
pub struct Posts {}

#[derive(Debug, Clone)]
pub enum Message {
    OpenSettings,
}

impl Posts {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::OpenSettings => println!("dupa"),
        }
        Command::none()
    }

    pub fn view(&self) -> Element<Message> {
        let btn = button("settings").on_press(Message::OpenSettings);

        Container::new(btn).into()
    }
}

impl Default for Posts {
    fn default() -> Self {
        Self::new()
    }
}
