use iced::widget::{button, column, row, text_input, Container, Space};
use iced::{executor, Length};
use iced::{Application, Command, Element, Theme};

use crate::api::{login, Instance, Instances};
use crate::settings::{Settings, JWT};

pub struct Lemnux {
    instance_domain: String,
    instance: Option<Instance>,
    username_field: String,
    password_field: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    NotFound,
    FetchInstance,
    Instances(Instances),
    DomainName(String),
    Username(String),
    Password(String),
    Login,
    Logged(Option<JWT>),
}

impl Application for Lemnux {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Lemnux, Command<Self::Message>) {
        (
            Lemnux {
                instance_domain: String::new(),
                instance: None,
                username_field: String::new(),
                password_field: String::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Lemnux")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::NotFound => Command::none(),
            Message::FetchInstance => {
                if !self.instance_domain.is_empty() {
                    return Command::perform(Instances::new(), |result| match result {
                        Ok(res) => Message::Instances(res),
                        Err(_) => Message::NotFound,
                    });
                }
                Command::none()
            }
            Message::Instances(inst) => {
                if !self.instance_domain.is_empty() {
                    self.instance = inst
                        .federated_instances
                        .linked
                        .into_iter()
                        .find(|fed| fed.domain == self.instance_domain)
                }

                if self.instance.is_some() {
                    let instance = self.instance.as_ref().unwrap();

                    let settings = Settings {
                        user: None,
                        jwt: None,
                        instance: Some(instance.clone()),
                    };

                    confy::store("lemnux", "instance", settings).unwrap();
                }

                Command::none()
            }
            Message::DomainName(domain_name) => {
                self.instance_domain = domain_name;
                Command::none()
            }
            Message::Username(user) => {
                self.username_field = user;
                Command::none()
            }
            Message::Password(pwd) => {
                self.password_field = pwd;
                Command::none()
            }
            Message::Login => {
                if !self.username_field.is_empty() && !self.password_field.is_empty() {
                    Command::perform(
                        login(
                            self.username_field.clone().into(),
                            self.password_field.clone().into(),
                            None,
                        ),
                        Message::Logged,
                    )
                } else {
                    Command::none()
                }
            }
            Message::Logged(jwt) => {
                let settings = Settings {
                    user: None,
                    jwt,
                    instance: None,
                };

                confy::store("lemnux", "jwt", settings).unwrap();

                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let search = text_input("Search for instance domain", &self.instance_domain)
            .on_input(Message::DomainName);

        let fetcher = button("Fetch instances").on_press(Message::FetchInstance);

        let row = row!(search, fetcher).spacing(30);

        let spacer = Space::new(Length::Fixed(30.), Length::Fixed(30.));

        let username_field =
            text_input("Username", &self.username_field).on_input(Message::Username);

        let password_field = text_input("Password", &self.password_field)
            .secure(true)
            .on_input(Message::Password);

        let login_btn = button("Log in")
            .on_press(Message::Login)
            .width(Length::Fill);

        let col = column!(username_field, password_field, login_btn).spacing(8);

        let content = column!(row, spacer, col);

        Container::new(content).padding(30).into()
    }
}
