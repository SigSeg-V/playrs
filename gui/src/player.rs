use std::time::Duration;

use gstreamer::ClockTime;
use iced::{
    executor, time,
    widget::{column, container},
    Application, Command, Element, Length, Subscription, Theme,
};
use playback::{playback::Sink, Status};
use rfd::FileDialog;

use crate::components;

pub struct Player {
    pub sink: Sink,
}

#[derive(Debug, Clone)]
pub enum Message {
    PlayPausePressed,
    StopPressed,
    ForwardPressed,
    BackwardPressed,
    Seek { seek_to: ClockTime },
    OpenFile,
    Tick,
}

impl Application for Player {
    type Message = Message;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                sink: Sink::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Playrs")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::PlayPausePressed => {
                if self.sink.playlist.is_empty(){
                    return Command::none()
                }
                self.sink.status = match self.sink.status {
                    // set to pause
                    Status::Playing => {
                        self.sink.pause_sound();
                        Status::Paused
                    }
                    // set to play
                    _ => {
                        self.sink.play_sound();
                        Status::Playing
                    }
                };
            }
            Message::StopPressed => {
                self.sink.status = Status::Stopped;
                self.sink.stop_sound();
            }
            Message::ForwardPressed => todo!(),
            Message::BackwardPressed => todo!(),
            Message::Seek { seek_to } => {
                self.sink.seek_absolute(seek_to);
            },
            Message::OpenFile => {
                self.sink
                    .add_to_queue(FileDialog::new().set_directory("~").pick_files());
            }
            Message::Tick => {
                self.sink.position = self.sink.get_position();
                self.sink.duration = self.sink.get_duration();
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let content = components::control_panel(
            &self.sink.status,
            &self.sink,
        );
        column!(
            container(content)
                .width(Length::Fill)
                //.height(Length::Fill)
                .center_x(),
            //.center_y(),
            components::playlist_table(&self.sink.playlist),
        )
        .into()
    }

    type Executor = executor::Default;

    type Theme = Theme;

    type Flags = ();

    fn subscription(&self) -> Subscription<Message> {
        match self.sink.status {
            Status::Playing => time::every(Duration::from_millis(100)).map(|_| Message::Tick),
            _ => Subscription::none(),
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new(()).0
    }
}
