use std::path::PathBuf;

use iced::widget::{button, column, row};
use iced::{alignment, Element, Length};
use iced::{
    widget::{text, Text},
    Font,
};

use crate::player::Message;
use playback::Status;
const ICON_FONT: Font = Font::External {
    name: "Nerd Font",
    bytes: include_bytes!(r"../fonts/Font Awesome 6 Free-Solid-900.otf"),
};

pub fn icon(unicode: char) -> Text<'static> {
    text(unicode.to_string()).font(ICON_FONT)
}

pub fn play_button<'a>(status: &Status) -> Element<'a, Message> {
    let button_icon = match status {
        Status::Playing => '\u{f04c}',
        _ => '\u{f04b}',
    };

    button(
        icon(button_icon)
            .width(Length::Fill)
            .height(Length::Fill)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center),
    )
    .on_press(Message::PlayPausePressed)
    .padding(0)
    .width(30)
    .height(30)
    .into()
}

pub fn stop_button<'a>(status: &Status) -> Element<'a, Message> {
    button(
        icon('\u{f04d}')
            .width(Length::Fill)
            .height(Length::Fill)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center),
    )
    .on_press(Message::StopPressed)
    .padding(0)
    .width(30)
    .height(30)
    .into()
}

pub fn open_file_dialog_button<'a>() -> Element<'a, Message> {
    button("Open File").on_press(Message::OpenFile).into()
}

pub fn play_text<'a>(status: &Status) -> Element<'a, Message> {
    text(match status {
        Status::Playing => "Playing",
        Status::Paused => "Paused",
        Status::Stopped => "Stopped",
    })
    .into()
}

pub fn playlist_table<'a>(values: &Vec<PathBuf>) -> Element<'a, Message> {
    column(
        values
            .iter()
            .map(|x| {
                text(
                    x.to_str()
                        .unwrap()
                        .split(std::path::MAIN_SEPARATOR_STR)
                        .last()
                        .unwrap(),
                )
                .into()
            })
            .collect::<Vec<Element<'a, Message>>>(),
    )
    .into()
}

pub fn control_panel<'a>(
    status: &Status,
    position: &String,
    duration: &String,
) -> Element<'a, Message> {
    let elems: Vec<Element<'a, Message>> = vec![
        stop_button(status),
        play_button(status),
        text(position).into(),
        text(duration).into(),
        open_file_dialog_button(),
    ];
    row(elems).spacing(50).into()
}
