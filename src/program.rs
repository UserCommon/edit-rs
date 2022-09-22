use std::borrow::{Borrow, BorrowMut};
use crossterm::cursor::CursorShape;
use std::io::Result;
use std::process::exit;
use crate::{utils, cursor::Cursor, events::EventMgr, renderer, config::Config, renderer::RenderMgr, Todo};
use crate::files::FileMgr;


pub struct Program {
    pub cfg: Config,
    cursor: Cursor,
    render: RenderMgr,
    event: EventMgr,
    pub file: FileMgr,
}

impl Program {
    pub fn builder() -> ProgramBuilder {
        ProgramBuilder::default()
    }

    pub fn run(&mut self) -> Result<()>{
        self.render.enter_canvas()?;
        self.render.set_draw_data(self.file.get_text());
        loop {
            self.render.draw(&self.cfg)?;
            self.event.event_manager()?;
            self.handle_events()?;
        }

    }

    fn handle_events(&mut self) -> Result<()> {
        let mut event;
        for _ in 0..self.event.queue.len() {
            event = self.event.queue.pop_back();
            if let Some(e) = event {
                self.match_event(e)?;
            } else {
                break;
            }
        }
        Ok(())
    }

    fn match_event(&mut self, event: Todo) -> Result<()> {
        match event {
            Todo::Quit => {
                self.render.exit()?;
                exit(0);
            },
            Todo::Resize(row, col) => {
                self.render.set_size((row, col));
            },
            _ => ()
        }
        Ok(())
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
            file: FileMgr::new()
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
        let mut cursor = Cursor::builder().shape(CursorShape::Line).build();
        let cfg = Config::builder().build();


        ProgramBuilder {
            cursor,
            cfg,
        }
    }
}