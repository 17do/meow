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
        stdout.execute(terminal::Clear(ClearType::All))?;

        Ok(())
    }
    pub fn end() -> Result<(), Box<dyn Error>> {
        let mut stdout = io::stdout();
        stdout.execute(terminal::LeaveAlternateScreen)?;
        Ok(())
    }
}
