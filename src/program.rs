use std::borrow::{Borrow, BorrowMut};
use crossterm::cursor::CursorShape;
use crossterm::terminal::size;
use std::io::Result;
use std::process::exit;
use crate::renderer::{DataReadMethods, DataModifyMethods};
use crate::terminal::Terminal;
use crate::{
    utils,
    cursor::{
        *
    },
    events::EventMgr,
    renderer,
    config::Config,
    renderer::RenderMgr,
    Events,
    Direction
};

use crate::files::FileMgr;
use std::thread;


pub struct Program {
    pub cfg: Config,
    render: RenderMgr,
    event: EventMgr,
    pub file: FileMgr,
}

// TODO: Сделать короче это...
// Проверку на то, можно ли шевелить курсором
impl Program {
    pub fn builder() -> ProgramBuilder {
        ProgramBuilder::default()
    }

    pub fn run(&mut self) -> Result<()>{
        //render initialization & configuration.
        self.render.enter_canvas()?;
        //self.render.set_raw_data(self.file.get_text());
        self.render.set_config(self.cfg.clone());

        //cursor configuration.
        self.from_file_start();
        //self.render.form_data(); -- Здесь можно запарсить осн документ, а шапку и футер оставить динамическим
        loop {
            self.render.update()?;
            self.event.event_manager()?; // <- | --BOTTLENECK)
            self.handle_events()?;
            Terminal::clear();
        }
        
    }

    fn handle_events(&mut self) -> Result<()> {
        for event in self.event.queue.get().clone() {
            self.match_event(&event)?;
        }
        self.event.queue.clear();
        Ok(())
    }

    fn can_move(&self, direction: Direction) -> bool {
        let pos = self.get_pos();
        match direction {
            Direction::Up => (pos.1 - 1) >= self.cfg.header_height,
            Direction::Down => (pos.1 + 1) <= size().unwrap().1 - self.cfg.header_height - self.cfg.footer_height,
            Direction::Left => !((pos.0 - 1) <= self.cfg.padding_size),
            Direction::Right => {
                let row = &self.render.get_data()[pos.1 as usize];
                ((pos.0 + 1) < (row.len()) as u16) && ((pos.0 + 1) < size().unwrap().0)
            },
        }
    }

    fn match_event(&mut self, event: &Events) -> Result<()> {
        let (row, col) = self.render.terminal.cursor.get_pos();
        let (row, col) = ((row - self.cfg.padding_size - 1) as usize, (col - self.cfg.header_height) as usize); 
        match event {
            Events::Quit => {
                self.exit()?;
            },
            Events::Resize(ref row,  ref col) => {
                self.render.terminal.set_size((*row, *col));
                Terminal::clear();
            },
            Events::MoveDown if self.can_move(Direction::Down) => {
                self.move_cursor(Direction::Down);
            },
            Events::MoveUp if self.can_move(Direction::Up) => {
                self.move_cursor(Direction::Up);
            },
            Events::MoveLeft if self.can_move(Direction::Left) => {
                self.move_cursor(Direction::Left);
            },
            Events::MoveRight if self.can_move(Direction::Right) => {
                self.move_cursor(Direction::Right);
            },
            
            Events::Erase if self.can_move(Direction::Left)=> {
                let (row, col) = (row, col);
                
                if row > 0 {
                    let data;
                    let line = format!(
                        "{}{}",
                        &data[col][0..(row - 1)],
                        &data[col][row..] // Кириллица не поддерживается. Нужно сделать отдельную структуру под хранение чаров
                    ).chars().collect();
                    self.render.data.edit_line(col, line) 
                    Terminal::clear();
                    self.move_cursor(Direction::Left);
                
                }
            },

            Events::Write(char) => {
        
                self.render.raw_data[col] = format!(
                    "{}{}{}",
                    &self.render.raw_data[col][0..row],
                    char,
                    &self.render.raw_data[col][row..]
                );
                self.move_cursor(Direction::Right);
            },

            Events::Space if self.can_move(Direction::Right) => {
                self.render.raw_data[col] = format!(
                    "{}{}{}",
                    &self.render.raw_data[col][0..row],
                    " ",
                    &self.render.raw_data[col][row..],
                );
                self.move_cursor(Direction::Right);
            },
            _ => ()
        }
        Ok(())
    }

    fn exit(&mut self) -> Result<()> {
        self.render.exit()?;
        exit(0);
    }
}

impl CursorReadMethods for Program {
    fn get_pos(&self) -> (u16, u16) {
        self.render.terminal.cursor.get_pos()
    }

    fn shape(&self) -> CursorShape {
        self.render.terminal.cursor.shape()
    }

    fn is_blinking(&self) -> bool {
        self.render.terminal.cursor.is_blinking()
    }
}

impl CursorMoveMethods for Program {
    fn move_cursor(&mut self, direction: Direction) {
        self.render.terminal.move_cursor(direction);
    }

    fn from_line_start(&mut self) {
        self.render.terminal.from_line_start();
    }

    fn from_file_start(&mut self) {
        self.render.terminal.from_file_start();
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
