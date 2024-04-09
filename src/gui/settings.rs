#![allow(clippy::large_enum_variant)]

use iced::{
    widget::{button, column, scrollable, text, text_input, Container, Space},
    Command, Element, Length,
};

use crate::{
    api::{login, Instance, Instances},
    settings::JWT,
};

#[derive(Debug, Clone)]
pub struct Settings {
    user_domain: String,
    instance: Option<Instance>,
    instances: Option<Vec<Instance>>,
    search_results: Option<Vec<Instance>>,
    username_field: String,
    password_field: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    NotFound,
    SetInstance(Instance),
    Instances(Instances),
    DomainName(String),
    Username(String),
    Password(String),
    Login,
    Logged(Option<JWT>),
}

impl Settings {
    pub fn new() -> Self {
        Self {
            user_domain: String::new(),
            instance: None,
            instances: None,
            search_results: None,
            username_field: String::new(),
            password_field: String::new(),
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::NotFound => Command::none(),
            Message::Instances(inst) => {
                self.instances = Some(inst.federated_instances.linked);

                if self.instance.is_some() {
                    let instance = self.instance.as_ref().unwrap();

                    let settings = crate::settings::Settings {
                        user: None,
                        jwt: None,
                        instance: Some(instance.clone()),
                    };

                    confy::store("lemnux", "instance", settings).unwrap();
                }

                Command::none()
            }
            Message::SetInstance(inst) => {
                self.instance = Some(inst);

                if self.instance.is_some() {
                    let instance = self.instance.as_ref().unwrap();

                    let settings = crate::settings::Settings {
                        user: None,
                        jwt: None,
                        instance: Some(instance.clone()),
                    };

                    confy::store("lemnux", "instance", settings).unwrap();
                }

                Command::none()
            }
            Message::DomainName(domain_name) => {
                self.user_domain = domain_name;

                if self.user_domain.len() == 3 && self.instances.is_none() {
                    return Command::perform(Instances::new(), |result| match result {
                        Ok(res) => Message::Instances(res),
                        Err(_) => Message::NotFound,
                    });
                }

                if self.user_domain.len() >= 3 && self.instances.is_some() {
                    let mut domains: Vec<Instance> = Vec::new();

                    self.instances
                        .clone()
                        .unwrap()
                        .into_iter()
                        .for_each(|item| {
                            if item.domain.contains(&self.user_domain) {
                                domains.push(item)
                            }
                        });

                    self.search_results = Some(domains);
                };

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
                let settings = crate::settings::Settings {
                    user: None,
                    jwt,
                    instance: None,
                };

                confy::store("lemnux", "jwt", settings).unwrap();

                Command::none()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let search = text_input("Search for instance domain", &self.user_domain)
            .on_input(Message::DomainName);

        let mut list = column!().padding(10.);

        if self.search_results.is_some() {
            for instance in self.search_results.clone().unwrap() {
                let item = button(text(instance.domain.to_string()))
                    .on_press(Message::SetInstance(instance))
                    .width(Length::Fill);
                list = list.push(item);
            }
        }

        let scrollable_list = scrollable(list)
            .width(Length::Fill)
            .height(Length::Fixed(100.));

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

        let content = column!(search, scrollable_list, spacer, col);

        Container::new(content).into()
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}
