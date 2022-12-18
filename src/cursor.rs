use crossterm::cursor::{self, CursorShape, MoveTo};
use crossterm::terminal::size;
use crossterm::execute;
use crate::Config;
use crate::utils::Direction;


pub trait CursorReadMethods {
    fn get_pos(&self) -> (u16, u16);
    fn is_blinking(&self) -> bool;
    fn shape(&self) -> CursorShape;
}

pub trait CursorMoveMethods {
    fn move_cursor(&mut self, direction: Direction);
    fn from_line_start(&mut self);
    fn from_file_start(&mut self);
}

pub trait CursorManipulateMethods {

    fn set_blink(&mut self, blink: bool);
    fn set_shape(&mut self, shape: CursorShape);
    fn update_config(&mut self, cfg: Config); 
}

/// Cursor struct
#[derive(Clone)]
pub struct Cursor {
    pub pos: (u16, u16),
    blinking: bool,
    shape: CursorShape,
    cfg: Config
}

impl Cursor {
    pub fn new() -> Self {
        Cursor::default()
    }

    // TODO! Change move cursor to right(So you can't move if there is no characters in line)
    /*
    fn can_move(&self, direction: &Direction) -> bool {
        match direction {
            Direction::Up => ((self.pos.1 - 1) >= self.cfg.header_height),
            Direction::Down => ((self.pos.1 + 1) <= size().unwrap().1 - self.cfg.header_height - self.cfg.footer_height),
            Direction::Left => !((self.pos.0 - 1) <= self.cfg.padding_size),
            Direction::Right => (self.pos.0 + 1) < size().unwrap().0,
        }
    }
    */

}

impl CursorMoveMethods for Cursor {
    fn move_cursor(&mut self, direction: Direction) {
       match direction {
            Direction::Up => self.pos.1 -= 1,
            Direction::Down => self.pos.1 += 1,
            Direction::Right => self.pos.0 += 1,
            Direction::Left => self.pos.0 -= 1
        } 
    }

    fn from_file_start(&mut self) {
        self.pos.0 = self.cfg.padding_size + 1;
        self.pos.1 = self.cfg.header_height;
    }

    fn from_line_start(&mut self) {
        self.pos.0 = self.cfg.padding_size + 1;
    }
}

impl CursorManipulateMethods for Cursor {
    fn update_config(&mut self, cfg: Config) {
        self.cfg = cfg;
    }

    fn set_blink(&mut self, blink: bool) { self.blinking = blink; }

    fn set_shape(&mut self, shape: CursorShape) { self.shape = shape; }
}

impl CursorReadMethods for Cursor {
    fn get_pos(&self) -> (u16, u16) { self.pos }
    fn is_blinking(&self) -> bool { self.blinking }    
    fn shape(&self) -> CursorShape { self.shape }
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor {
            pos: (0, 0),
            blinking: false,
            shape: CursorShape::Line,
            cfg: Config::builder().build(),
        }
    }
}
