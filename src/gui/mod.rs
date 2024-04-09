#![allow(clippy::large_enum_variant)]

pub mod posts;
pub mod settings;

use iced::{
    executor,
    widget::{button, column, row, Container},
    Application, Command, Element, Theme,
};

use self::settings::Settings;

#[derive(Debug, Clone)]
pub enum Pages {
    Home(Box<posts::Posts>),
    Settings(Box<settings::Settings>),
}

pub struct Lemnux {
    pub page: Pages,
}

#[derive(Debug, Clone)]
pub enum Message {
    Home(posts::Message),
    Settings(settings::Message),
    GotoHome,
    OpenSettings,
}

impl Application for Lemnux {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Lemnux, Command<Self::Message>) {
        (
            Lemnux {
                page: Pages::Home(Box::new(posts::Posts::new())),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Lemnux")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Home(post_mess) => {
                let Pages::Home(home_page) = &mut self.page else {
                    return Command::none();
                };

                home_page.update(post_mess).map(Message::Home)
            }
            Message::Settings(opt) => {
                let Pages::Settings(settings_page) = &mut self.page else {
                    return Command::none();
                };

                settings_page.update(opt).map(Message::Settings)
            }
            Message::GotoHome => {
                self.page = Pages::Home(Box::new(posts::Posts::new()));

                Command::none()
            }
            Message::OpenSettings => {
                self.page = Pages::Settings(Box::new(Settings::new()));

                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let home_btn = button("Home").on_press(Message::GotoHome);
        let sett_btn = button("Settings").on_press(Message::OpenSettings);
        let topbar = row!(home_btn, sett_btn);

        let page = match &self.page {
            Pages::Home(posts) => posts.view().map(Message::Home),
            Pages::Settings(settings) => settings.view().map(Message::Settings),
        };

        let content = column!(topbar, page);

        Container::new(content).into()
    }
}
