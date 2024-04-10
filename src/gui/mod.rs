#![allow(clippy::large_enum_variant)]

pub mod posts;
pub mod settings;

use iced::{
    executor,
    widget::{button, column, row, Container},
    Application, Command, Element, Theme,
};
use lemmy_api_common::post::GetPostsResponse;

use self::settings::Settings;
use crate::api::{get_posts, Instances};

#[derive(Debug, Clone)]
pub enum Pages {
    Init,
    Home(Box<posts::Posts>),
    Settings(Box<settings::Settings>),
}

pub struct Lemnux {
    pub page: Pages,
    theme: Theme,
    posts: Option<GetPostsResponse>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Home(posts::Message),
    InitalizePosts(Option<GetPostsResponse>),
    Settings(settings::Message),
    GotoHome,
    OpenSettings,
    FetchedInstances(Instances),
}

impl Application for Lemnux {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Lemnux, Command<Self::Message>) {
        let load_posts = Command::perform(get_posts(None), Message::InitalizePosts);
        let theme = crate::settings::Settings::load_theme();

        (
            Lemnux {
                page: Pages::Init,
                theme,
                posts: None,
            },
            load_posts,
        )
    }

    fn theme(&self) -> Self::Theme {
        self.theme.clone()
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
            Message::OpenSettings => Command::perform(Instances::new(), Message::FetchedInstances),
            Message::FetchedInstances(inst) => {
                self.page =
                    Pages::Settings(Box::new(Settings::new(inst.federated_instances.linked)));

                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let home_btn = button("Lemnux")
            .on_press(Message::GotoHome)
            .width(92)
            .height(92);
        let sett_btn = button("Settings")
            .on_press(Message::OpenSettings)
            .width(92)
            .height(92);
        let topbar = row!(home_btn, sett_btn).spacing(8).height(100);

        let page = match &self.page {
            Pages::Init => row!().into(),
            Pages::Home(posts) => posts.view().map(Message::Home),
            Pages::Settings(settings) => settings.view().map(Message::Settings),
        };

        let content = column!(topbar, page);

        Container::new(content).into()
    }
}
