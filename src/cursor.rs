use std::io::Result;

use crossterm::cursor::{self, CursorShape, MoveTo};
use crossterm::execute;
use crate::Config;
use crate::utils::Direction;

/// Cursor struct
#[derive(Clone)]
pub struct Cursor {
    pub pos: (u16, u16),
    blinking: bool,
    shape: CursorShape,
    cfg: Config
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

    pub fn move_c(&mut self, direction: Direction) {
        let flag = self.can_move(&direction);

        if flag {
            match direction {
                Direction::Up => self.pos.1 -= 1,
                Direction::Down => self.pos.1 += 1,
                Direction::Right => self.pos.0 += 1,
                Direction::Left => self.pos.0 -= 1
            }
        }
    }

    fn can_move(&self, direction: &Direction) -> bool {
        match direction {
            Direction::Up => ((self.pos.1 - 1) >= self.cfg.header_height),
            Direction::Down => true,
            _ => {true}
        }
    }

    pub fn from_file_start(&mut self) {
        self.pos.0 = self.cfg.padding_size + 1;
        self.pos.1 = self.cfg.header_height;
    }

    pub fn from_line_start(&mut self) {
        self.pos.0 = self.cfg.padding_size + 1;
    }

    pub fn update_config(&mut self, cfg: Config) {
        self.cfg = cfg;
    }
}

pub struct CursorBuilder {
    pos: (u16, u16),
    blinking: bool,
    shape: CursorShape,
    cfg: Option<Config>
}

impl CursorBuilder {
    pub fn build(&mut self) -> Cursor {
        Cursor {
            pos: self.pos,
            blinking: self.blinking,
            shape: self.shape,
            cfg: self.cfg.clone().expect("No cfg in CursorBuilder")
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

    pub fn config(&mut self, cfg: Config) -> &mut Self {
        self.cfg = Some(cfg);
        self
    }
}

impl Default for CursorBuilder {
    fn default() -> Self {
        let pos = (0, 0);
        let blinking = false;
        let shape = CursorShape::Line;
        let cfg = Config::builder().build();

        CursorBuilder {
            pos,
            blinking,
            shape,
            cfg: Some(cfg)
        }
    }
}