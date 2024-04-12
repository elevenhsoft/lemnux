use iced::{
    widget::{
        button, column, row,
        scrollable::{scroll_to, AbsoluteOffset, Id, Scrollable},
        text, Container,
    },
    Command, Element, Length,
};
use iced_aw::Card;
use lemmy_api_common::{
    lemmy_db_schema::ListingType, lemmy_db_views::structs::PaginationCursor, post::GetPostsResponse,
};

use crate::api::get_posts;

#[derive(Debug)]
pub struct Posts {
    post_list: Option<GetPostsResponse>,
    post_cards: Vec<PostCard>,
    pagination: Option<PaginationCursor>,
    type_: Option<ListingType>,
    next_page: Option<PaginationCursor>,
}

#[derive(Debug, Clone)]
pub enum PostFetching {
    NextPage(Option<PaginationCursor>),
    Loaded(Option<GetPostsResponse>),
    Idle,
}

#[derive(Debug, Clone)]
pub enum Message {
    PostStatus(PostFetching),
}

#[derive(Debug)]
struct PostCard {
    name: String,
    creator: String,
    body: String,
}

fn convert_postsview_to_card(
    post_list: Option<GetPostsResponse>,
) -> (Vec<PostCard>, Option<PaginationCursor>) {
    let mut result = Vec::new();
    let mut next_page: Option<PaginationCursor> = None;

    if let Some(posts) = post_list {
        posts.posts.into_iter().for_each(|item| {
            let body = if let Some(text) = item.post.body {
                text
            } else {
                String::from("Read more...")
            };

            let author = format!("@{}", item.creator.name);

            let card_data = PostCard {
                name: item.post.name,
                creator: author,
                body,
            };
            result.push(card_data);
        });

        next_page = posts.next_page;
    };

    (result, next_page)
}

impl Posts {
    pub fn new(type_: Option<ListingType>, post_list: Option<GetPostsResponse>) -> Self {
        let post_cards = convert_postsview_to_card(post_list.clone());

        Self {
            post_list,
            post_cards: post_cards.0,
            pagination: None,
            type_,
            next_page: post_cards.1,
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::PostStatus(fetcher) => match fetcher {
                PostFetching::NextPage(page_cursor) => {
                    self.pagination = page_cursor;

                    Command::perform(get_posts(self.type_, self.pagination.to_owned()), |ret| {
                        Message::PostStatus(PostFetching::Loaded(ret))
                    })
                }
                PostFetching::Loaded(posts) => {
                    self.post_list = posts;

                    if let Some(np) = &self.post_list {
                        self.next_page = np.next_page.to_owned();
                    }

                    self.post_cards = convert_postsview_to_card(self.post_list.clone()).0;

                    scroll_to(Id::new("PostsContainer"), AbsoluteOffset { x: 0., y: 0. })
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
            let body_row = row!(text(&post.body)).spacing(30).padding(30);

            col = col.push(Card::new(title_row, body_row));
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
