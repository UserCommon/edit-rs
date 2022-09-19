use crossterm::cursor::CursorShape;
use crate::utils::Direction;

/// Cursor struct
#[derive(Clone)]
pub struct Cursor {
    pos: (u16, u16),
    blinking: bool,
    shape: CursorShape
}

impl Cursor {
    pub fn builder() -> CursorBuilder {
        CursorBuilder::default()
    }

    pub fn get_pos(&self) -> (u16, u16) { self.pos }

    pub fn is_blinking(&self) -> bool { self.blinking }

    pub fn set_blink(&mut self, blink: bool) { self.blinking = blink; }

    pub fn shape(&self) -> CursorShape { self.shape }

    pub fn set_shape(&mut self, shape: CursorShape) { self.shape = shape; }

    pub fn move_cursor(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.pos.1 += 1,
            Direction::Down => self.pos.1 -= 1,
            Direction::Right => self.pos.0 += 1,
            Direction::Left => self.pos.0 += 1
        }
    }
}

pub struct CursorBuilder {
    pos: (u16, u16),
    blinking: bool,
    shape: CursorShape
}

impl CursorBuilder {
    pub fn build(&mut self) -> Cursor {
        Cursor {
            pos: self.pos,
            blinking: self.blinking,
            shape: self.shape
        }
    }

    pub fn pos(&mut self, pos: (u16, u16)) -> &mut Self {
        self.pos = pos;
        self
    }

    pub fn blinking(&mut self, is_blinking: bool) -> &mut Self {
        self.blinking = is_blinking;
        self
    }

    pub fn shape(&mut self, shape: CursorShape) -> &mut Self {
        self.shape = shape;
        self
    }
}

impl Default for CursorBuilder {
    fn default() -> Self {
        let pos = (0, 0);
        let blinking = false;
        let shape = CursorShape::Line;

        CursorBuilder {
            pos,
            blinking,
            shape
        }
    }
}