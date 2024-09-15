#![allow(warnings)]

use std::{path::PathBuf, time::Duration};
mod base;
mod draw;
mod meow_music;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Source};
pub struct MeowMusics<'a> {
    music: &'a str,
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
}
pub struct App;
pub struct MeowBox {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    text: String,
}

#[test]
fn test() {
    App::new().unwrap();
    loop {
        App::clear().unwrap();
        MeowBox::new(5, 5, 15, 15).draw().unwrap();
    }
    App::end();
}
