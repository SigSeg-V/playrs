use std::path::PathBuf;

use iced::widget::{button, column, progress_bar, row, slider};
use iced::{alignment, Element, Length};
use iced::{
    widget::{text, Text},
    Font,
};
use playback::playback::Sink;

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

pub fn playlist_table<'a>(values: &[PathBuf]) -> Element<'a, Message> {
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

pub fn top_bar<'a>(duration: u64, position: u64)-> Element<'a, Message> {
    slider(0..=duration, position, |_|{}).into()
}

pub fn control_panel<'a>(
    status: &Status,
    sink: &Sink,
) -> Element<'a, Message> {
    let position = sink.get_position();
    let duration = sink.get_duration();
    let position_u64 = sink.get_position_u64();
    let duration_u64 = sink.get_duration_u64();

    let pbar = top_bar(duration_u64, position_u64);

    let elems: Vec<Element<'a, Message>> = vec![
        stop_button(status),
        play_button(status),
        text(position).into(),
        text(duration).into(),
        open_file_dialog_button(),
    ];
    column!(pbar,row(elems).spacing(50)).into()
}
