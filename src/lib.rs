#![allow(warnings)]

use meow_music::MeowMusicSetup;

mod meow_music;

pub struct MeowMusics {
    music: String,
}

#[test]
fn main() {
    MeowMusicSetup::setup()
        .unwrap()
        .play("./meow_music.rs")
        .unwrap();
}
