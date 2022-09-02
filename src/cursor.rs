use crossterm::cursor::CursorShape;
use crate::utils::Direction;

/// Cursor struct
pub struct Cursor {
    pos: (u16, u16),
    blinking: bool,
    shape: CursorShape
}

impl Cursor {
    pub fn new(is_blinking: bool, cursor_shape: CursorShape) -> Self {
        Cursor {
            pos: (0, 0),
            blinking: is_blinking,
            shape: cursor_shape
        }
    }

    pub fn get_pos(&self) -> (u16, u16) { self.pos }

    pub fn is_blinking(&self) -> bool { self.blinking }

    pub fn shape(&self) -> CursorShape { self.shape }

    pub fn move_cursor(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.pos.1 += 1,
            Direction::Down => self.pos.1 -= 1,
            Direction::Right => self.pos.0 += 1,
            Direction::Left => self.pos.0 += 1
        }
    }
}