use gui::Lemnux;
use iced::{Application, Settings};

pub mod api;
pub mod gui;
pub mod settings;

pub fn main() -> iced::Result {
    Lemnux::run(Settings::default())
}
