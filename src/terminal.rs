use std::io::Write;
use std::io::stdout;
use std::io::Result;
use crossterm::execute;
use crossterm::terminal::{self, size};
use crossterm::style::{self, Color};
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

    pub fn clear() {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
    }

    pub fn hide_cursor() {
        // Hide the text cursor
        execute!(stdout(), crossterm::cursor::Hide).unwrap();
    }
    pub fn show_cursor() {
        execute!(stdout(), crossterm::cursor::Show).unwrap();
    }

    pub fn set_background(color: Color) {
        execute!(stdout(), style::SetBackgroundColor(color)).unwrap();
    }

    pub fn flush() {
        // Flush the screen to prevent weird behaviour
        stdout().flush().unwrap();
    }
}