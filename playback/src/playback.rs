use std::{path::PathBuf};


use gstreamer as gst;
use gst::prelude::*;
use gstreamer_player as gst_player;

use gst_player::Player;

pub struct Sink {
    pub player: Player,
    pub playlist: Vec<PathBuf>,
    pub current_file: usize,
    pub status: Status,
    pub position: String,
    pub duration: String,
}

pub enum Status {
    Playing,
    Paused,
    Stopped,
}

impl Default for Sink {
    fn default() -> Self {
        // init audio streams
        match gst::init() {
            Ok(_) => (),
            Err(_) => {
                eprintln!("Error initializing GStreamer")
            },
        }

        let dispatcher = gst_player::PlayerGMainContextSignalDispatcher::new(None);
        let player = gst_player::Player::new(
            None::<gst_player::PlayerVideoRenderer>,
            Some(dispatcher.upcast::<gst_player::PlayerSignalDispatcher>()),
        );

        Self {
            player,
            playlist: vec![],
            current_file: 0,
            status: Status::Stopped,
            position: "--:--:--".to_string(),
            duration: "--:--:--".to_string(),
        }
    }
}

impl Sink {
    pub fn add_to_queue(&mut self, files: Option<Vec<PathBuf>>) {
        // set index for the current file to the first song if not already set
        if self.playlist.is_empty() {
            self.current_file = 0;
        }

        match files {
            Some(files) => for file in files {
                self.playlist.push(PathBuf::from("file://".to_string() + file.to_str().unwrap()));
            },
            None => {
            let file = PathBuf::from(r"file:///Users/charlie/Documents/Rust/playrs/src-tauri/assets/Scarlet Fire.mp3");
            self.playlist.push(file);
            },
        }
    }

    pub fn pop_playlist(&mut self) {
        self.playlist.pop();
    }

    pub fn load_file(&self) {
        self.player.set_uri(self.playlist[self.current_file].to_str());
    }

    pub fn play_sound(&self) {
        if self.player.uri().is_none() {
            self.load_file();
        }
        self.player.play();
    }

    pub fn pause_sound(&self) {
        self.player.pause();
    }

    pub fn stop_sound(&self) {
        self.player.stop();
    }

    pub fn get_duration(&self) -> String {
        match self.player.duration() {
            Some(dur) => {
                let hr = dur.hours();
                let min = dur.minutes() - hr * 60;
                let sec = dur.seconds() - (min * 60) - (hr * 60 * 60);
                format!("{:<02}:{:<02}:{:<02}", hr, min, sec)
            },
            None => "--:--:--".to_string(),
        }
    }

    pub fn get_position(&self) -> String {
        match self.player.position() {
            Some(pos) => {
                let hr = pos.hours();
                let min = pos.minutes() - hr * 60;
                let sec = pos.seconds() - (min * 60) - (hr * 60 * 60);
                format!("{:<02}:{:<02}:{:<02}", hr, min, sec)
            },
            None => "--:--:--".to_string(),
        }
    }
}
