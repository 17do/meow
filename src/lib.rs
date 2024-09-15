#![allow(warnings)]

use std::path::PathBuf;
mod meow_music;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Source};
pub struct MeowMusics<'a> {
    music: &'a str,
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
}

#[test]
fn test() {
    let s = MeowMusics::new("/home/inado/Music/Kan Saete Kuyashiiwa.mp3").unwrap();
    println!("Time: {:?}", s.metadata());
    std::thread::sleep(s.play().unwrap());
}
