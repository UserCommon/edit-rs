use std::borrow::{Borrow, BorrowMut};
use crossterm::cursor::CursorShape;
use std::io::Result;
use crate::{
    utils,
    cursor::Cursor,
    events::EventMgr,
    renderer,
    config::Config,
    renderer::RenderMgr
};


pub struct Program {
    pub cfg: Config,
    cursor: Cursor,
    render: RenderMgr,
    event: EventMgr
}

impl Program {
    pub fn builder() -> ProgramBuilder {
        ProgramBuilder::default()
    }

    pub fn run(&mut self) -> Result<()>{
        loop {
            self.render.draw(&self.cfg)?;
            self.event.event_manager()?;
        }
    }
}

pub struct ProgramBuilder {
    pub cfg: Config,
    pub cursor: Cursor,
}

impl ProgramBuilder {
    pub fn build(&mut self) -> Program {
        Program {
            cfg: self.cfg.clone(),
            cursor: self.cursor.clone(),
            render: RenderMgr::new().unwrap(),
            event: EventMgr::new(),
        }
    }

    pub fn cfg(&mut self, cfg: Config) -> &mut Self {
        self.cfg = cfg;
        self
    }

    pub fn cursor(&mut self, cursor: Cursor) -> &mut Self {
        self.cursor = cursor;
        self
    }
}

impl Default for ProgramBuilder {
    fn default() -> Self {
        let mut cursor = Cursor::builder().shape(CursorShape::Block).build();
        let cfg = Config::builder().build();


        ProgramBuilder {
            cursor,
            cfg,
        }
    }
}