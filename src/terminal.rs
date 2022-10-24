use std::io::Stdout;
use std::io::Result;
use crossterm::terminal::size;
use crate::{cursor, Cursor, Direction};


pub struct Terminal {
    pub rows: u16,
    pub columns: u16,
    pub cursor: Cursor,
}

impl Terminal {
    pub fn new() -> Result<Self> {
        let size = size()?;
        Ok(Terminal {
            rows: size.0,
            columns: size.1,
            cursor: Cursor::builder().build()
        })
    }

    pub fn get_size(&self) -> (u16, u16) {
        (self.rows, self.columns)
    }

    pub fn set_size(&mut self, size: (u16, u16)) {
        self.rows = size.0;
        self.columns = size.1;
    }

    pub fn move_cursor(&mut self, direction: Direction) {
        self.cursor.move_c(direction);
    }
}