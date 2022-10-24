use std::borrow::{Borrow, BorrowMut};
use crossterm::cursor::CursorShape;
use std::io::Result;
use std::process::exit;
use crate::{utils, cursor::Cursor, events::EventMgr, renderer, config::Config, renderer::RenderMgr, Events, Direction};
use crate::files::FileMgr;


pub struct Program {
    pub cfg: Config,
    render: RenderMgr,
    event: EventMgr,
    pub file: FileMgr,
}

impl Program {
    pub fn builder() -> ProgramBuilder {
        ProgramBuilder::default()
    }

    pub fn run(&mut self) -> Result<()>{
        //render initialization & configuration.
        self.render.enter_canvas()?;
        self.render.set_raw_data(self.file.get_text());
        self.render.set_config(self.cfg.clone());

        //cursor configuration.
        self.render.terminal.cursor.from_file_start();
        //self.render.form_data(); -- Здесь можно запарсить осн документ, а шапку и футер оставить динамическим
        loop {
            self.render.draw()?;
            self.event.event_manager()?; // <- | --BOTTLENECK ИЛИ НЕТ)
            self.handle_events()?;       // <- /
        }

    }

    fn handle_events(&mut self) -> Result<()> {
        for event in &self.event.queue.get_events().clone() {
            self.match_event(event)?;
        }
        self.event.queue.commit();
        Ok(())
    }

    fn match_event(&mut self, event: &Events) -> Result<()> {
        match event {
            Events::Quit => {
                self.exit()?;
            },
            Events::Resize(ref row,  ref col) => {
                self.render.terminal.set_size((*row, *col));
            },
            Events::MoveDown => {
                self.render.terminal.move_cursor(Direction::Down);
            }
            Events::MoveUp => {
                self.render.terminal.move_cursor(Direction::Up);
            }
            Events::MoveLeft => {
                self.render.terminal.move_cursor(Direction::Left);
            }
            Events::MoveRight => {
                self.render.terminal.move_cursor(Direction::Right);
            }
            _ => ()
        }
        Ok(())
    }

    fn exit(&mut self) -> Result<()> {
        self.render.exit()?;
        exit(0);
    }
}

pub struct ProgramBuilder {
    pub cfg: Config,
}

impl ProgramBuilder {

    pub fn build(&mut self) -> Program {
        Program {
            cfg: self.cfg.clone(),
            render: RenderMgr::new(),
            event: EventMgr::new(),
            file: FileMgr::new()
        }
    }

    pub fn cfg(&mut self, cfg: Config) -> &mut Self {
        self.cfg = cfg;
        self
    }

}

impl Default for ProgramBuilder {
    fn default() -> Self {
        let cfg = Config::builder().build();

        ProgramBuilder {
            cfg,
        }
    }
}