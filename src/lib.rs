#![allow(warnings)]

use std::path::PathBuf;
mod base;
mod meow_music;
use base::App;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Source};
pub struct MeowMusics<'a> {
    music: &'a str,
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
}

#[test]
fn test() {
    App::new().unwrap();
    let s = MeowMusics::new("/home/inado/Music/Kan Saete Kuyashiiwa.mp3").unwrap();
    println!("Time: {:?}", s.metadata());
    s.play().unwrap();
    std::thread::sleep(s.metadata().unwrap());

    App::end().unwrap();
}
