use iced::{
    widget::{button, column, row, scrollable, text, Container, Rule},
    Command, Element, Length,
};
use lemmy_api_common::{lemmy_db_views::structs::PaginationCursor, post::GetPostsResponse};

use crate::api::get_posts;

#[derive(Debug, Clone)]
pub struct Posts {
    pub state: Message,
    pub post_list: Option<GetPostsResponse>,
    pub pagination: Option<PaginationCursor>,
}

#[derive(Debug, Clone)]
pub enum PostFetching {
    Loading,
    NextPage(Option<PaginationCursor>),
    Loaded(Option<GetPostsResponse>),
    Idle,
}

#[derive(Debug, Clone)]
pub enum Message {
    PostStatus(PostFetching),
}

impl Posts {
    pub fn new() -> Self {
        Self {
            state: Message::PostStatus(PostFetching::Loading),
            post_list: None,
            pagination: None,
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::PostStatus(fetcher) => match fetcher {
                PostFetching::Loading => {
                    self.post_list = None;

                    if self.post_list.is_none() {
                        return Command::perform(get_posts(self.pagination.to_owned()), |ret| {
                            Message::PostStatus(PostFetching::Loaded(ret))
                        });
                    }

                    Command::none()
                }
                PostFetching::NextPage(page_cursor) => {
                    self.pagination = page_cursor;

                    Command::perform(get_posts(self.pagination.to_owned()), |ret| {
                        Message::PostStatus(PostFetching::Loaded(ret))
                    })
                }
                PostFetching::Loaded(posts) => {
                    self.state = Message::PostStatus(PostFetching::Idle);
                    self.post_list = posts;
                    Command::none()
                }
                PostFetching::Idle => Command::none(),
            },
        }
    }

    pub fn view(&self) -> Element<Message> {
        let mut row = row!();
        let mut col = column!().spacing(30);

        let load_btn = button("Load").on_press(Message::PostStatus(PostFetching::Loading));

        col = col.push(load_btn);

        if self.post_list.is_none() {
            let loading_mess = text("Loading...");
            row = row.push(loading_mess);
        } else {
            let posts = self.post_list.clone().unwrap().posts;
            let next_page = self.post_list.clone().unwrap().next_page;

            for post in posts.into_iter() {
                let name = text(post.post.name);
                let body = if let Some(body) = post.post.body {
                    text(body)
                } else {
                    text(String::new())
                };

                let ruler = Rule::horizontal(2);
                let post_card = column!(name, body, ruler).spacing(10).padding(30);

                col = col.push(post_card);
            }

            let next_page_btn = button("Next Page")
                .on_press(Message::PostStatus(PostFetching::NextPage(next_page)))
                .width(Length::Fill);

            col = col.push(next_page_btn);
        };

        row = row.push(col);

        let scrollable = scrollable(row);

        Container::new(scrollable).into()
    }
}

impl Default for Posts {
    fn default() -> Self {
        Self::new()
    }
}
