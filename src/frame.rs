use crate::{MeowPosition, Place};
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::{size, ClearType};
use std::error::Error;
use std::io::{self, Write};
use std::u16;

impl Place {
    pub fn center() -> Result<MeowPosition, Box<dyn Error>> {
        let (columns, rows) = size()?;

        // 中央の座標を計算
        let center_x = columns / 2;
        let center_y = rows / 2;

        Ok(MeowPosition {
            x: center_x,
            y: center_y,
        })
    }
}
