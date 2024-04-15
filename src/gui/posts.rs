use iced::{
    advanced::image::Handle,
    widget::{
        button, column, row,
        scrollable::{scroll_to, AbsoluteOffset, Id, Scrollable},
        text, Container, Image,
    },
    Command, Element, Length,
};
use iced_aw::Card;
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
}

#[derive(Debug, Clone)]
pub struct PostCard {
    _url: String,
    name: String,
    creator: String,
    body: String,
    thumbnail: Option<Handle>,
}

pub async fn convert_postsview_to_card(item: PostView) -> PostCard {
    let url = if let Some(u) = item.post.url {
        u.to_string()
    } else {
        String::new()
    };
    let author = format!("@{}", item.creator.name);
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

    PostCard {
        _url: url,
        name: item.post.name,
        creator: author,
        body,
        thumbnail,
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
        }
    }

    pub fn view(&self) -> Element<Message> {
        let mut col = column!().spacing(60).padding(30);

        for post in &self.post_cards {
            let title_row = row!(button(text(&post.creator)), text(&post.name))
                .spacing(20)
                .align_items(iced::Alignment::Center);
            let body_row = if post.thumbnail.is_some() {
                row!(Image::new(post.thumbnail.clone().unwrap()))
            } else {
                row!(text(&post.body)).spacing(30).padding(30)
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
