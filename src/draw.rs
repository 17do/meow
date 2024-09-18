use crate::{Line, MeowPosition, Place, Square};
use crossterm::{
    cursor, execute,
    style::{self, Print},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::{
    error::Error,
    io::{stdout, Write},
};

impl Square {
    pub fn new(x_: u16, y_: u16, w: u16, h: u16) -> Self {
        Square {
            x: x_,
            y: y_,
            width: w,
            height: h,
            text: "".to_string(),
        }
    }
    pub fn new_set(xy: MeowPosition, w: u16, h: u16) -> Self {
        let (x_, y_) = MeowPosition::ret(&xy);
        Square {
            x: x_,
            y: y_,
            width: w,
            height: h,
            text: "".to_string(),
        }
    }
    pub fn draw(&self) -> Result<(), Box<dyn Error>> {
        let mut stdout = stdout();

        // 上の辺を描画
        stdout.execute(cursor::MoveTo(self.x, self.y))?;
        stdout.execute(Print("─".repeat(self.width as usize)))?;

        // 左右の辺を描画
        for i in 1..self.height - 1 {
            // 左の辺
            stdout.execute(cursor::MoveTo(self.x, self.y + i))?;
            stdout.execute(Print("│"))?;

            // 右の辺
            stdout.execute(cursor::MoveTo(self.x + self.width - 1, self.y + i))?;
            stdout.execute(Print("│"))?;
        }

        // 下の辺を描画
        stdout.execute(cursor::MoveTo(self.x, self.y + self.height - 1))?;
        stdout.execute(Print("─".repeat(self.width as usize)))?;

        // 四隅を描画
        stdout.execute(cursor::MoveTo(self.x, self.y))?;
        stdout.execute(Print("┌"))?; // 左上
        stdout.execute(cursor::MoveTo(self.x + self.width - 1, self.y))?;
        stdout.execute(Print("┐"))?; // 右上
        stdout.execute(cursor::MoveTo(self.x, self.y + self.height - 1))?;
        stdout.execute(Print("└"))?; // 左下
        stdout.execute(cursor::MoveTo(
            self.x + self.width - 1,
            self.y + self.height - 1,
        ))?;
        stdout.execute(Print("┘"))?; // 右下

        stdout.flush()?;
        Ok(())
    }
}

impl Line {
    pub fn new() -> Self {
        println!("-------------");
        Self
    }
}
