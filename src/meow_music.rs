use crate::MeowMusics;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Source};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

pub struct MeowMusicSetup {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
}

impl MeowMusics {
    pub fn new(file_name: &str) -> Self {
        MeowMusics {
            music: file_name.to_string(),
        }
    }
    pub fn setup_play(&self) -> Result<(), Box<dyn Error>> {
        let (_stream, stream_handle) = OutputStream::try_default()?;

        let file = File::open(self.music.as_str())?;
        let source = Decoder::new(BufReader::new(file))?;

        stream_handle.play_raw(source.convert_samples())?;

        Ok(())
    }
}

impl MeowMusicSetup {
    pub fn setup() -> Result<MeowMusicSetup, Box<dyn Error>> {
        let (stream, _handle) = OutputStream::try_default()?;
        Ok(MeowMusicSetup {
            _stream: stream,
            stream_handle: _handle,
        })
    }
    pub fn play(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let file = File::open(file_path)?;
        let source = Decoder::new(BufReader::new(file))?;

        self.stream_handle.play_raw(source.convert_samples())?;

        Ok(())
    }
}
