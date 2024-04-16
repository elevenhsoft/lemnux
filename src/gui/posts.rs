use iced::{
    advanced::image::Handle,
    theme,
    widget::{
        button, column, horizontal_rule, row,
        scrollable::{scroll_to, AbsoluteOffset, Id, Scrollable},
        text, Container, Image,
    },
    Command, Element, Length,
};
use iced_aw::{badge, BadgeStyles, Card};
use lemmy_api_common::{
    lemmy_db_schema::ListingType,
    lemmy_db_views::structs::{PaginationCursor, PostView},
    post::GetPostsResponse,
};

use crate::api::{get_posts, load_img_to_memory};

#[derive(Debug)]
pub struct Posts {
    type_: Option<ListingType>,
    post_cards: Vec<PostCard>,
    next_page: Option<PaginationCursor>,
}

#[derive(Debug, Clone)]
pub enum PostFetching {
    NextPage,
    LoadedResponse(GetPostsResponse),
    LoadedPost(PostCard),
    Idle,
}

#[derive(Debug, Clone)]
pub enum Message {
    PostStatus(PostFetching),
    OpenPost(String),
}

#[derive(Debug, Clone)]
pub struct PostCard {
    url: String,
    name: String,
    creator: String,
    body: String,
    thumbnail: Option<Handle>,
    updated: String,
}

pub async fn convert_postsview_to_card(item: PostView) -> PostCard {
    let url = if let Some(u) = item.post.url {
        u.to_string()
    } else {
        String::new()
    };
    let author = format!("{}@{}", item.creator.name, item.community.title);
    let body = if let Some(text) = item.post.body {
        text
    } else {
        String::from("Read more...")
    };
    let thumbnail = if let Some(url) = item.post.thumbnail_url {
        Some(Handle::from_memory(
            load_img_to_memory(&url.to_string()).await,
        ))
    } else {
        None
    };
    let updated = if let Some(updated) = item.post.updated {
        updated.to_rfc2822()
    } else {
        item.post.published.to_rfc2822()
    };

    PostCard {
        url,
        name: item.post.name,
        creator: author,
        body,
        thumbnail,
        updated,
    }
}

impl Posts {
    pub fn new(
        type_: Option<ListingType>,
        post_cards: Vec<PostCard>,
        next_page: Option<PaginationCursor>,
    ) -> Self {
        Self {
            type_,
            post_cards,
            next_page,
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::PostStatus(fetcher) => match fetcher {
                PostFetching::NextPage => {
                    Command::perform(get_posts(self.type_, self.next_page.to_owned()), |ret| {
                        Message::PostStatus(PostFetching::LoadedResponse(ret))
                    })
                }
                PostFetching::LoadedResponse(posts) => {
                    self.next_page = posts.next_page;
                    self.post_cards.clear();

                    let mut cmds = vec![scroll_to(
                        Id::new("PostsContainer"),
                        AbsoluteOffset { x: 0., y: 0. },
                    )];

                    for item in posts.posts.into_iter() {
                        cmds.push(Command::perform(convert_postsview_to_card(item), |card| {
                            Message::PostStatus(PostFetching::LoadedPost(card))
                        }));
                    }

                    Command::batch(cmds)
                }
                PostFetching::LoadedPost(card) => {
                    self.post_cards.push(card);
                    Command::none()
                }
                PostFetching::Idle => Command::none(),
            },
            Message::OpenPost(link) => {
                println!("{}", link);

                Command::none()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let mut col = column!().spacing(60).padding(30);

        for post in &self.post_cards {
            let title_row = column!(
                button(text(&post.name))
                    .style(theme::Button::Secondary)
                    .width(Length::Fill)
                    .on_press(Message::OpenPost(post.url.clone())),
                horizontal_rule(1),
                row!(
                    badge(text(&post.creator)).style(BadgeStyles::Primary),
                    badge(text(&post.updated)).style(BadgeStyles::Info)
                )
                .spacing(10)
            )
            .spacing(15);

            let body_row = if post.thumbnail.is_some() {
                Container::new(Image::new(post.thumbnail.clone().unwrap()))
                    .width(Length::Fill)
                    .center_x()
                    .center_y()
            } else {
                Container::new(text(&post.body))
                    .width(Length::Fill)
                    .center_x()
                    .center_y()
                    .padding(30)
            };

            col = col.push(Card::new(title_row, body_row));
        }

        let next_page_btn = button("Next Page")
            .on_press(Message::PostStatus(PostFetching::NextPage))
            .width(Length::Fill);

        col = col.push(next_page_btn);

        let scrollable = Scrollable::new(col).id(Id::new("PostsContainer"));

        Container::new(scrollable).into()
    }
}
