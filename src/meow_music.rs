use crate::MeowMusics;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Source};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::time::Duration;

impl<'a> MeowMusics<'a> {
    pub fn new(file_name: &'a str) -> Result<Self, Box<dyn Error>> {
        let path = Path::new(file_name);
        if !path.exists() {
            return Err("No such file or directory found".into());
        }

        let (_stream, stream_handle) = OutputStream::try_default()?;
        Ok(MeowMusics {
            music: file_name,
            _stream,
            stream_handle,
        })
    }

    pub fn play(&self) -> Result<Duration, Box<dyn Error>> {
        let file = File::open(Path::new(self.music))?;
        let source = Decoder::new(BufReader::new(file))?;
        let mut time: Duration = Duration::new(1, 1);

        if let Some(duration) = source.total_duration() {
            time = duration;
        }

        self.stream_handle.play_raw(source.convert_samples())?;

        Ok(time)
    }
}
