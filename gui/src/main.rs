// iced

use iced::{Settings, Application};
use iced::window;

// includes
mod components;
mod player;
use player::Player;
pub fn main() -> iced::Result {
    let window_settings = window::Settings{min_size: Some((600, 400)), ..Default::default()};

    Player::run(Settings{window: window_settings, ..Default::default()})
}

