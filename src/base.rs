use crossterm::{
    cursor,
    event::{self, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::{
    error::Error,
    io::{self, Write},
};

pub struct App;

impl App {
    pub fn new() -> Result<(), Box<dyn Error>> {
        let mut stdout = io::stdout();

        stdout.execute(terminal::Clear(ClearType::All))?;
        stdout.execute(terminal::EnterAlternateScreen)?;
        Ok(())
    }
}
