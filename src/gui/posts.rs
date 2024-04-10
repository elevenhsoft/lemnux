use iced::{
    widget::{
        button, column, row,
        scrollable::{scroll_to, AbsoluteOffset, Id, Scrollable},
        text, Container, Rule,
    },
    Command, Element, Length,
};
use lemmy_api_common::{lemmy_db_views::structs::PaginationCursor, post::GetPostsResponse};

use crate::api::get_posts;

#[derive(Debug, Clone)]
pub struct Posts {
    pub post_list: Option<GetPostsResponse>,
    pub pagination: Option<PaginationCursor>,
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

impl Posts {
    pub fn new(post_list: Option<GetPostsResponse>) -> Self {
        Self {
            post_list,
            pagination: None,
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::PostStatus(fetcher) => match fetcher {
                PostFetching::NextPage(page_cursor) => {
                    self.pagination = page_cursor;

                    Command::perform(get_posts(self.pagination.to_owned()), |ret| {
                        Message::PostStatus(PostFetching::Loaded(ret))
                    })
                }
                PostFetching::Loaded(posts) => {
                    self.post_list = posts;

                    scroll_to(Id::new("PostsContainer"), AbsoluteOffset { x: 0., y: 0. })
                }
                PostFetching::Idle => Command::none(),
            },
        }
    }

    pub fn view(&self) -> Element<Message> {
        let mut row = row!().spacing(4);
        let mut col = column!().spacing(4);

        if self.post_list.is_some() {
            let posts = self.post_list.clone().unwrap().posts;
            let next_page = self.post_list.clone().unwrap().next_page;

            for post in posts.into_iter() {
                let name = button(text(post.post.name));
                let body = if let Some(body) = post.post.body {
                    text(body)
                } else {
                    text(String::new())
                };

                let ruler = Rule::horizontal(2);
                let post_card = column!(name, body, ruler).spacing(10);

                col = col.push(post_card);
            }

            let next_page_btn = button("Next Page")
                .on_press(Message::PostStatus(PostFetching::NextPage(next_page)))
                .width(Length::Fill);

            col = col.push(next_page_btn);
        };

        row = row.push(col);

        let scrollable = Scrollable::new(row).id(Id::new("PostsContainer"));

        Container::new(scrollable).into()
    }
}
