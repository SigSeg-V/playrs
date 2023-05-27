use std::time::Duration;

use gstreamer::ClockTime;
use iced::{Application, Command, Element, widget::{row, container}, Alignment, executor, Theme, Subscription, time, Length};
use native_dialog::FileDialog;
use playback::{playback::Sink, Status};

use crate::components::*;


pub struct Player {
    pub sink: Sink,
}

#[derive(Debug, Clone)]
pub enum Message {
    PlayPausePressed,
    ForwardPressed,
    BackwardPressed,
    Seek{seek_to: ClockTime},
    OpenFile,
    Tick,
}

impl Application for Player {
    type Message = Message;

    fn new(_flags: ()) -> (Self, Command<Message>)  {
        (Self{sink: Sink::default()}, Command::none())
    }

    fn title(&self) -> String {
        String::from("Playrs")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::PlayPausePressed => {
                self.sink.status = match self.sink.status {
                    // set to pause
                    Status::Playing => {
                        self.sink.pause_sound();
                        Status::Paused
                    },
                    // set to play
                    _ => {
                        self.sink.play_sound();
                        Status::Playing
                    },
                };
            },
            Message::ForwardPressed => todo!(),
            Message::BackwardPressed => todo!(),
            Message::Seek { seek_to } => todo!(),
            Message::OpenFile => {
                self.sink.add_to_queue(match FileDialog::new().set_location("~").show_open_multiple_file(){
                    Ok(val) => Some(val),
                    Err(_) => None,
                });
            }
            Message::Tick => {
                self.sink.position = self.sink.get_position();
            },
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {

        let content: Element<Message> = row![
        res_text(&self.sink.position),
        play_button(&self.sink.status),
        play_text(&self.sink.status),
        open_file_dialog_button(),
        playlist_table(&self.sink.playlist)
        ]
            .spacing(50)
            .padding(20)
            .align_items(Alignment::Center)
            .into();

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    type Executor = executor::Default;

    type Theme = Theme;

    type Flags = ();

    fn subscription(&self) -> Subscription<Message> {
        match self.sink.status {
            Status::Playing => time::every(Duration::from_millis(10)).map(|_| Message::Tick),
            _ => Subscription::none(),
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new(()).0
    }
}