use crate::App;
use crossterm::{
    cursor,
    event::{self, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::{
    error::Error,
    io::{self, stdout, Write},
    time::Duration,
};

impl App {
    pub fn new() -> Result<(), Box<dyn Error>> {
        let mut stdout = io::stdout();

        stdout.execute(terminal::EnterAlternateScreen)?;
        Ok(())
    }
    pub fn end() -> Result<(), Box<dyn Error>> {
        let mut stdout = io::stdout();
        stdout.execute(terminal::LeaveAlternateScreen)?;
        Ok(())
    }
    pub fn fps(fps: f32) -> Result<(), Box<dyn Error>> {
        let frame_duration = Duration::from_secs_f64(1.0 / fps);
        let mut stdout = io::stdout();
        stdout.execute(terminal::Clear(ClearType::All))?;
        std::thread::sleep(frame_duration);
        Ok(())
    }
}
