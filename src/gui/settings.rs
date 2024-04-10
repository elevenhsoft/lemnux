#![allow(
    clippy::large_enum_variant,
    clippy::unnecessary_to_owned,
    clippy::to_string_in_format_args
)]

use std::fs;

use iced::{
    widget::{button, column, combo_box, combo_box::State, text, text_input, Container},
    Command, Element, Length,
};

use crate::{
    api::{login, Instance},
    settings::{AppTheme, Preferences, User, JWT},
};

#[derive(Debug, Clone)]
pub struct Settings {
    instance: Option<Instance>,
    instances_to_search: State<Instance>,
    user_selected_instance: Option<Instance>,
    app_theme_chooser: State<AppTheme>,
    user_theme: AppTheme,
    username_field: String,
    password_field: String,
    user: Option<User>,
}

#[derive(Debug, Clone)]
pub enum Message {
    NotFound,
    SetInstance(Instance),
    UserSelectedInstance(Instance),
    SetTheme(AppTheme),
    Username(String),
    Password(String),
    Login,
    Logged(Option<JWT>),
    Logout,
}

impl Settings {
    pub fn new(instances: Vec<Instance>) -> Self {
        let user = if let Ok(config) = confy::load::<crate::settings::Settings>("lemnux", "user") {
            config.user
        } else {
            None
        };

        let themes = AppTheme::to_vec();
        let app_theme_chooser = State::new(themes.clone());
        let user_theme = if let Ok(config) =
            confy::load::<crate::settings::Preferences>("lemnux", "preferences")
        {
            config.theme
        } else {
            themes[0].clone()
        };

        Self {
            instance: None,
            instances_to_search: State::new(instances),
            user_selected_instance: None,
            app_theme_chooser,
            user_theme,
            username_field: String::new(),
            password_field: String::new(),
            user,
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::NotFound => Command::none(),
            Message::SetInstance(inst) => {
                self.instance = Some(inst);

                if self.instance.is_some() {
                    let instance = self.instance.as_ref().unwrap();

                    let settings = crate::settings::Settings {
                        instance: Some(instance.clone()),
                        ..Default::default()
                    };

                    confy::store("lemnux", "instance", settings).unwrap();
                }

                Command::none()
            }
            Message::UserSelectedInstance(inst) => {
                self.user_selected_instance = Some(inst);

                Command::none()
            }
            Message::SetTheme(theme) => {
                self.user_theme = theme.clone();
                let prefs = Preferences { theme };
                confy::store("lemnux", "preferences", prefs).unwrap();

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
                    user: Some(User::new(self.username_field.clone().into(), jwt, true)),
                    ..Default::default()
                };

                confy::store("lemnux", "user", settings).unwrap();

                Command::none()
            }
            Message::Logout => {
                let config_path = confy::get_configuration_file_path("lemnux", "user");

                if let Ok(file) = config_path {
                    if file.exists() {
                        fs::remove_file(file).unwrap();
                    }
                }

                self.instance = None;
                self.user = None;

                Command::none()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let mut content = column!();

        content = content.push(combo_box(
            &self.app_theme_chooser,
            "Select app theme",
            Some(&self.user_theme),
            Message::SetTheme,
        ));

        content = content.push(
            combo_box(
                &self.instances_to_search,
                "Select your instance you want to work with",
                self.user_selected_instance.as_ref(),
                Message::SetInstance,
            )
            .on_option_hovered(Message::UserSelectedInstance),
        );

        if self.instance.is_some() || self.user.is_some() {
            let username_field =
                text_input("Username", &self.username_field).on_input(Message::Username);

            let password_field = text_input("Password", &self.password_field)
                .secure(true)
                .on_input(Message::Password);

            let login_btn = button("Log in")
                .on_press(Message::Login)
                .width(Length::Fill);

            let col = if self.user.is_some() && self.user.as_ref().unwrap().is_logged {
                let welcome_message = text(format!(
                    "Welcome, {}",
                    self.user.as_ref().unwrap().username.to_string()
                ));
                let logout_btn = button("Logout").on_press(Message::Logout);
                column!(welcome_message, logout_btn)
            } else {
                column!(username_field, password_field, login_btn).spacing(8)
            };

            content = content.push(col);
        }

        Container::new(content).into()
    }
}
