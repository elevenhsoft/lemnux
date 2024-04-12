use iced::{
    advanced::image::{Bytes, Handle},
    widget::{
        button, column, row,
        scrollable::{scroll_to, AbsoluteOffset, Id, Scrollable},
        text, Container, Image,
    },
    Command, Element, Length,
};
use iced_aw::Card;
use lemmy_api_common::{
    lemmy_db_schema::ListingType, lemmy_db_views::structs::PaginationCursor, post::GetPostsResponse,
};

use crate::api::{get_posts, load_img_to_memory};

#[derive(Debug)]
pub struct Posts {
    type_: Option<ListingType>,
    post_cards: Option<Vec<PostCard>>,
    next_page: Option<PaginationCursor>,
}

#[derive(Debug, Clone)]
pub enum PostFetching {
    NextPage(Option<PaginationCursor>),
    Loaded((Vec<PostCard>, Option<PaginationCursor>)),
    Idle,
}

#[derive(Debug, Clone)]
pub enum Message {
    PostStatus(PostFetching),
}

#[derive(Debug, Clone)]
pub struct PostCard {
    url: String,
    name: String,
    creator: String,
    body: String,
    thumbnail: Option<Bytes>,
}

pub async fn convert_postsview_to_card(
    post_list: Option<GetPostsResponse>,
) -> (Vec<PostCard>, Option<PaginationCursor>) {
    let mut result = Vec::new();
    let mut next_page: Option<PaginationCursor> = None;

    if let Some(posts) = post_list.clone() {
        for item in posts.posts {
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
                Some(load_img_to_memory(&url.to_string()).await)
            } else {
                None
            };

            let card_data = PostCard {
                url,
                name: item.post.name,
                creator: author,
                body,
                thumbnail,
            };
            result.push(card_data);
        }

        next_page = posts.next_page;
    };

    (result, next_page)
}

impl Posts {
    pub fn new(
        type_: Option<ListingType>,
        post_cards: Option<Vec<PostCard>>,
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
                PostFetching::NextPage(page_cursor) => {
                    self.next_page = page_cursor;

                    Command::perform(get_posts(self.type_, self.next_page.to_owned()), |ret| {
                        Message::PostStatus(PostFetching::Loaded(ret))
                    })
                }
                PostFetching::Loaded(posts) => {
                    self.post_cards = Some(posts.0);
                    self.next_page = posts.1;

                    scroll_to(Id::new("PostsContainer"), AbsoluteOffset { x: 0., y: 0. })
                }
                PostFetching::Idle => Command::none(),
            },
        }
    }

    pub fn view(&self) -> Element<Message> {
        let mut col = column!().spacing(60).padding(30);

        if let Some(cards) = &self.post_cards {
            for post in cards {
                let title_row = row!(button(text(&post.creator)), text(&post.name))
                    .spacing(20)
                    .align_items(iced::Alignment::Center);
                let body_row = if post.thumbnail.is_some() {
                    row!(Image::new(Handle::from_memory(
                        post.thumbnail.clone().unwrap()
                    )))
                } else {
                    row!(text(&post.body)).spacing(30).padding(30)
                };

                col = col.push(Card::new(title_row, body_row));
            }
        }

        let next_page_btn = button("Next Page")
            .on_press(Message::PostStatus(PostFetching::NextPage(
                self.next_page.clone(),
            )))
            .width(Length::Fill);

        col = col.push(next_page_btn);

        let scrollable = Scrollable::new(col).id(Id::new("PostsContainer"));

        Container::new(scrollable).into()
    }
}
