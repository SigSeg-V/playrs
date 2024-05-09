// iced

use iced::{Application, Settings, Size};
use iced::window;

// includes
mod components;
mod player;
use player::Player;
pub fn main() -> iced::Result {
    let window_settings = window::Settings{min_size: Some(Size::new(600., 400.)), max_size: Some(Size::new(1000.,1000.)), ..Default::default()};

    Player::run(Settings{window: window_settings, ..Default::default()})
}

