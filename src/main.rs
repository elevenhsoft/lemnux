use gui::App;
use iced::{Application, Settings};

pub mod api;
pub mod gui;
pub mod settings;

pub fn main() -> iced::Result {
    App::run(Settings::default())
}
