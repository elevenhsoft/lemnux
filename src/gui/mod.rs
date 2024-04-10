#![allow(clippy::large_enum_variant)]

pub mod posts;
pub mod settings;

use iced::{
    executor,
    widget::{button, column, row, Container},
    Application, Command, Element, Theme,
};
use lemmy_api_common::post::GetPostsResponse;

use crate::api::get_posts;

use self::settings::Settings;

#[derive(Debug, Clone)]
pub enum Pages {
    Init,
    Home(Box<posts::Posts>),
    Settings(Box<settings::Settings>),
}

pub struct Lemnux {
    pub page: Pages,
    posts: Option<GetPostsResponse>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Home(posts::Message),
    InitalizePosts(Option<GetPostsResponse>),
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
        let load_posts = Command::perform(get_posts(None), Message::InitalizePosts);

        (
            Lemnux {
                page: Pages::Init,
                posts: None,
            },
            load_posts,
        )
    }

    fn title(&self) -> String {
        String::from("Lemnux")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::GotoHome => {
                self.page = Pages::Home(Box::new(posts::Posts::new(self.posts.clone())));
                Command::none()
            }
            Message::Home(post_mess) => {
                let Pages::Home(home_page) = &mut self.page else {
                    return Command::none();
                };

                home_page.update(post_mess).map(Message::Home)
            }
            Message::InitalizePosts(posts) => {
                self.posts = posts;
                self.page = Pages::Home(Box::new(posts::Posts::new(self.posts.clone())));
                Command::none()
            }
            Message::Settings(opt) => {
                let Pages::Settings(settings_page) = &mut self.page else {
                    return Command::none();
                };

                settings_page.update(opt).map(Message::Settings)
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
            Pages::Init => row!().into(),
            Pages::Home(posts) => posts.view().map(Message::Home),
            Pages::Settings(settings) => settings.view().map(Message::Settings),
        };

        let content = column!(topbar, page);

        Container::new(content).into()
    }
}
