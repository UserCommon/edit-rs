use crate::config::Config;

use std::io::{Stdout, stdout, Write, Result};


use crossterm::{execute, terminal::{
    self,
    size,
    EnterAlternateScreen,
    LeaveAlternateScreen,
    enable_raw_mode,
    disable_raw_mode
}, style::{
    Stylize,
    Color,
}, ExecutableCommand, style};
use crossterm::cursor::{DisableBlinking, EnableBlinking, MoveTo, SetCursorShape};
use crossterm::style::style;
use crate::Cursor;


// TODO! Refactor & add config field
pub struct RenderMgr {
    rows: u16,
    columns: u16,
    stdout: Stdout,
    draw_data: Option<Vec<String>>,
    cfg: Option<Config>
}

impl RenderMgr {
    pub fn new() -> Result<Self> {
        let (row, column) = size()?;
        Ok(RenderMgr {
            rows: row,
            columns: column,
            stdout: stdout(),
            draw_data: None,
            cfg: None,
        })
    }

    pub fn set_config(&mut self, cfg: Config) {
        self.cfg = Some(cfg);
    }

    pub fn set_size(&mut self, size: (u16, u16)) {
        self.rows = size.0;
        self.columns = size.1;
    }

    pub fn set_draw_data(&mut self, draw_data: Vec<String>) {
        self.draw_data = Some(draw_data);
    }

    pub fn enter_canvas(&mut self) -> Result<()> {
        self.stdout.execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        disable_raw_mode()?;
        self.stdout.execute(LeaveAlternateScreen)?;
        Ok(())
    }

    /// this function drawing things
    pub fn draw(&mut self, cursor: &Cursor) -> Result<()> {
        let cfg = self.get_cfg();
        execute!(&self.stdout, terminal::Clear(terminal::ClearType::All))?;
        execute!(&self.stdout, style::SetBackgroundColor(cfg.default_background_color))?;

        // draw_header
        self.draw_panel("header", true)?;

        if self.draw_data.is_some() {
            // draw document data
            self.draw_doc()?;
        } else {
            self.draw_splash_screen()?;
        }
        // draw footer
        self.draw_panel("footer", false)?;

        //self.draw_debug_data()?;

        // handle cursor
        let pos = cursor.get_pos();
        execute!(&self.stdout,
                    MoveTo(pos.0, pos.1),
                    SetCursorShape(cursor.shape()),
        )?;
        Ok(())
    }
    // Не работает
    fn draw_panel(&mut self, label: &str, is_header: bool) -> Result<()> {
        let to_draw = " ".repeat(self.rows as usize - label.len()) + label;
        self.write_on_bg(to_draw)?;


        let cfg = self.get_cfg();

        for _ in 0..if is_header {cfg.header_height} else {cfg.footer_height} - 1 {
            self.write_on_bg(" ".repeat(self.rows as usize))?;
        }
        Ok(())
    }

    fn draw_doc(&mut self) -> Result<()> {

        let mut line = 1;
        for data in self.draw_data.clone().unwrap() {
            let to_write = self.str_enumeration(line) + &data;
            line += 1;
            self.write_text(to_write)?;
        }

        if line < self.columns {
            for _ in 0..self.columns - line - 2 {
                let to_write = self.str_enumeration(line);
                self.write_text(to_write)?;
                line += 1;
            }
        }
        // Если осталось место дорисовать столько строк, сколько необходимо
        Ok(())
    }

    fn str_enumeration(&self, num: u16) -> String {
        let cfg = self.get_cfg();

        let str_num = num.to_string();
        " ".repeat(cfg.padding_size as usize - str_num.len())
            + &str_num
            + " "
    }

    fn draw_splash_screen(&mut self) -> Result<()> {


        Ok(())
    }

    fn write_on_bg(&mut self, data: String) -> Result<()> {
        let cfg = self.get_cfg();
        write!(self.stdout, "\r{}\n", data
            .on(cfg.default_background_color)
            .with(cfg.default_font_color)
        )?;
        Ok(())
    }

    fn write_text(&mut self, data: String) -> Result<()> {
        let cfg = self.get_cfg();
        write!(self.stdout, "\r{}\n", data
            .with(cfg.default_font_color)
        )?;
        Ok(())
    }

    fn draw_debug_data(&mut self) -> Result<()>{
        write!(self.stdout, "\r{:?}\n", &self.draw_data)?;
        Ok(())
    }

    fn get_cfg(&self) -> &Config {
        self.cfg.as_ref().unwrap()
    }
}