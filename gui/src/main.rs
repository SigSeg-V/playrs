use iced::widget::{button, column, text, text_input, row, container, Text};
use iced::{Alignment, Element, Sandbox, Length, Settings, Font, alignment};
use iced::window;
use iced_native::widget::Widget;

use playback::playback::PlayState;
use ClockTime;

pub fn main() -> iced::Result {
    let window_settings = window::Settings{min_size: Some((600, 400)), ..Default::default()};
    Counter::run(Settings{window: window_settings, ..Default::default()})
}

struct Counter {
    value: String,
    copied_value: String,

}

#[derive(Debug, Clone)]
enum Message {
    PlayPausePressed,
    ForwardPressed,
    BackwardPressed,
    Seek{seek_to: ClockTime},
}

impl Sandbox for PlayState {
    type Message = Message;

    fn new() -> Self {
        Self {
            sink: Mutex::new(
                    Sink{player,
                         playlist: vec![],
                         current_file: 0,
                         is_playing: false
                    })
        }
    }

    fn title(&self) -> String {
        String::from("Playrs")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TranslateHex => {
                self.copied_value = self.value.clone();
            }
            Message::ValueEdited(updated_str) => {
                self.value = updated_str;
            }
        }
    }

    fn view(&self) -> Element<Message> {

        let content: Element<Message> = row![hex_inp(&self.value), play_button(), res_text(&self.copied_value), ]
            .spacing(50)
            .padding(20)
            .align_items(Alignment::Center)
            .into();

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            //.max_width(800)
            .into()
    }
}



const ICON_FONT: Font = Font::External { name: "Nerd Font", bytes: include_bytes!(r"..\fonts\Fira Code Regular Nerd Font Complete.ttf") };

fn icon(unicode: char) -> Text<'static> {
    text(unicode.to_string())
        .font(ICON_FONT)
        .width(20)
        .horizontal_alignment(alignment::Horizontal::Center)
        .size(20)
}

fn play_button<'a>(is_playing: bool) -> Element<'a, Message> {
    match 


    button(icon(''))
        .on_press(Message::PlayPausePressed)
        .into()
}

fn hex_inp<'a>(value: &String) -> Element<'a, Message> {
    text_input("Raw Hex Here", &value)
            .on_input(Message::ValueEdited)
            .on_submit(Message::TranslateHex)
            .width(200)
            .into()
}

fn res_text<'a>(value: &String) -> Element<'a, Message> {
    text(&value).into()
}
