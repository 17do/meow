#![allow(warnings)]

use rodio::{Decoder, OutputStream, OutputStreamHandle, Source};
use std::{path::PathBuf, time::Duration};
mod base;
mod draw;
mod frame;
mod meow_music;
mod meowpositon;

pub struct MeowMusics<'a> {
    music: &'a str,
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
}
pub struct App;
pub struct Square {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    text: String,
}

pub struct Place;
pub struct MeowPosition {
    x: u16,
    y: u16,
}
pub struct Line;

#[macro_export]
macro_rules! input {
    ($s:expr) => {
        std::io::stdin().read_line($s).unwrap();
    };
}

#[test]
fn test() {
    App::new().unwrap();
    Square::new(5, 5, 3, 3).draw().unwrap();
    std::thread::sleep(Duration::from_secs(5));
    App::end();
}
