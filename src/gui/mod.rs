#![allow(clippy::large_enum_variant)]

pub mod posts;
pub mod settings;

use iced::{
    alignment, executor,
    widget::{column, text, Container},
    Application, Command, Element, Length, Theme,
};
use iced_aw::native::{TabBar, TabLabel};
use lemmy_api_common::{
    lemmy_db_schema::ListingType, lemmy_db_views::structs::PaginationCursor, post::GetPostsResponse,
};

use self::{
    posts::{convert_postsview_to_card, PostCard},
    settings::Settings,
};
use crate::api::{get_posts, Instance, Instances};

#[derive(Debug)]
pub enum Pages {
    Posts(posts::Posts),
    Settings(settings::Settings),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TabId {
    All,
    Local,
    Subscribed,
    Settings,
}

#[derive(Debug)]
pub struct Lemnux {
    page: Pages,
    active_tab: TabId,
    theme: Theme,
    posts_type: Option<ListingType>,
    instances: Vec<Instance>,
    post_cards: Vec<PostCard>,
    next_page: Option<PaginationCursor>,
}

pub enum App {
    Loading,
    Loaded(Lemnux),
}

#[derive(Debug)]
pub enum Message {
    Loaded(Lemnux),
    TabSelected(TabId),
    PostFetched(GetPostsResponse),
    PostRendered(PostCard),
    RenderPosts,
    Posts(posts::Message),
    Settings(settings::Message),
}

async fn load() -> Lemnux {
    let theme = crate::settings::Settings::load_theme();
    let posts_type = Some(ListingType::All);
    let posts = get_posts(posts_type, None).await;
    let instances = Instances::new().await.federated_instances.linked;
    let post_cards = Vec::new();

    Lemnux {
        page: Pages::Posts(posts::Posts::new(
            posts_type,
            post_cards.clone(),
            posts.next_page.clone(),
        )),
        active_tab: TabId::All,
        theme,
        posts_type,
        instances,
        post_cards,
        next_page: posts.next_page,
    }
}

impl Application for App {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (App::Loading, Command::perform(load(), Message::Loaded))
    }

    fn theme(&self) -> Self::Theme {
        if let App::Loaded(config) = &self {
            config.theme.clone()
        } else {
            crate::settings::Settings::load_theme()
        }
    }

    fn title(&self) -> String {
        String::from("Lemnux")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            App::Loading => {
                if let Message::Loaded(init) = message {
                    *self = App::Loaded(init)
                }
                Command::perform(
                    get_posts(Some(ListingType::All), None),
                    Message::PostFetched,
                )
            }
            App::Loaded(config) => match message {
                Message::TabSelected(tab) => {
                    config.active_tab = tab.clone();

                    match tab {
                        TabId::All => {
                            config.posts_type = Some(ListingType::All);

                            Command::perform(
                                get_posts(config.posts_type, None),
                                Message::PostFetched,
                            )
                        }
                        TabId::Local => {
                            config.posts_type = Some(ListingType::Local);

                            Command::perform(
                                get_posts(config.posts_type, None),
                                Message::PostFetched,
                            )
                        }
                        TabId::Subscribed => {
                            config.posts_type = Some(ListingType::Subscribed);

                            Command::perform(
                                get_posts(config.posts_type, None),
                                Message::PostFetched,
                            )
                        }
                        TabId::Settings => {
                            config.page =
                                Pages::Settings(Settings::new(config.instances.to_owned()));

                            Command::none()
                        }
                    }
                }
                Message::PostFetched(posts) => {
                    config.next_page = posts.next_page;

                    let cmds = posts.posts.into_iter().map(|item| {
                        Command::perform(convert_postsview_to_card(item), Message::PostRendered)
                    });

                    config.post_cards.clear();

                    Command::batch(cmds)
                }
                Message::PostRendered(card) => {
                    config.post_cards.push(card);

                    let object = posts::Posts::new(
                        config.posts_type,
                        config.post_cards.to_owned(),
                        config.next_page.to_owned(),
                    );

                    config.page = Pages::Posts(object);

                    Command::none()
                }
                Message::Posts(post_mess) => {
                    let Pages::Posts(home_page) = &mut config.page else {
                        return Command::none();
                    };

                    home_page.update(post_mess).map(Message::Posts)
                }
                Message::Settings(opt) => {
                    let Pages::Settings(settings_page) = &mut config.page else {
                        return Command::none();
                    };

                    if let settings::Message::SetTheme(theme) = &opt {
                        config.theme =
                            crate::settings::Settings::translate_app_theme(theme.to_owned());
                    };

                    settings_page.update(opt).map(Message::Settings)
                }
                _ => Command::none(),
            },
        }
    }

    fn view(&self) -> Element<Self::Message> {
        match self {
            App::Loading => Container::new(
                text("Loading...")
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .size(50),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_y()
            .center_x()
            .into(),
            App::Loaded(config) => {
                let tab_bar = TabBar::new(Message::TabSelected)
                    .push(TabId::All, TabLabel::Text(String::from("All")))
                    .push(TabId::Local, TabLabel::Text(String::from("Local")))
                    .push(
                        TabId::Subscribed,
                        TabLabel::Text(String::from("Subscribed")),
                    )
                    .push(TabId::Settings, TabLabel::Text(String::from("Settings")))
                    .set_active_tab(&config.active_tab);

                let page = match &config.page {
                    Pages::Posts(posts) => posts.view().map(Message::Posts),
                    Pages::Settings(settings) => settings.view().map(Message::Settings),
                };

                let content = column!(tab_bar, page);

                Container::new(content).into()
            }
        }
    }
}
