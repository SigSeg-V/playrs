use std::sync::Mutex;

pub use gstreamer::ClockTime;
use gstreamer_player as gst_player;
use gst_player::{Player};
use tauri::Window;

pub struct PlayState {
    pub sink: Mutex<Sink>,
}
pub struct Sink {
    pub player: Player,
    pub playlist: Vec<String>,
    pub current_file: usize,
    pub status: Status,
}

pub enum Status {
    Playing,
    Paused,
    Stopped,
}

#[derive(Clone, serde::Serialize)]
struct ClockTimePayload {
    pub hours: u64,
    pub mins: u64,
    pub secs: u64,
    pub msecs: u64,
}

trait Payload {
    fn new_payload(&self) -> ClockTimePayload;
}

impl Payload for ClockTime {
    fn new_payload(&self) -> ClockTimePayload {
        ClockTimePayload {
            hours: self.hours(),
            mins: self.minutes(),
            secs: self.seconds(),
            msecs: self.mseconds(),
        }
    }
}

pub fn add_to_queue(files: Vec<String>) {

    let mut sink = state.sink.lock().unwrap();

    // set index for the current file to the first song if not already set
    if sink.playlist.is_empty() {
        sink.current_file = 0;
    }

    // default music file for testing
    if files.is_empty() {
        let file = r"file:///Users/charlie/Documents/Rust/playrs/src-tauri/assets/Scarlet Fire.mp3".to_string();
        sink.playlist.push(file);
    }

    // coming from the file browser
    else {
        for file in files {
            sink.playlist.push("file://".to_string() + &file);
        }
    }
    // emit the changes to the playlist to the ui
    window.emit("update-playlist", sink.playlist.clone())
        .expect("Error updating playlist");
}

pub fn pop_playlist(state: PlayState) {
    let mut sink = state.sink.lock().unwrap();

    sink.playlist.pop();
    window.emit("update-playlist", sink.playlist.clone())
        .expect("Error updating playlist");
}

pub fn load_file(state: PlayState) {
    let sink = state.sink.lock().unwrap();
    sink.player.set_uri(Some(sink.playlist[sink.current_file].as_str()));
}

pub fn play_sound(state: PlayState) {
    let sink = state.sink.lock().unwrap();
    if sink.player.uri().is_none() {
        sink.player.set_uri(Some(sink.playlist[sink.current_file].as_str()));
    }
    sink.player.play();
}

pub fn pause_sound(state: PlayState) {
    let sink = state.sink.lock().unwrap();
    sink.player.pause();
}

pub fn stop_sound(state: PlayState) {
    let sink = state.sink.lock().unwrap();
    sink.player.stop();
}

pub fn get_duration(state: PlayState) {
    let sink = state.sink.lock().unwrap();

    if let Some(dur) = sink.player.duration() {
        window.emit("get-duration", dur.new_payload()).expect("Could not find duration");
    }
}

pub fn get_position(state: PlayState) {
    let sink = state.sink.lock().unwrap();

    if let Some(pos) = sink.player.position() {
        window.emit("get-position", pos.new_payload()).expect("Could not find position");
    }
}