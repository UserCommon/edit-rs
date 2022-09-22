use crate::config::Config;

use std::io::{Stdout, stdout, Write, Result};
use std::ptr::write;

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
use crossterm::style::style;


// TODO! Refactor & add config field
pub struct RenderMgr {
    rows: u16,
    columns: u16,
    stdout: Stdout,
    draw_data: Option<Vec<String>>
}

impl RenderMgr {
    pub fn new() -> Result<Self> {
        let (row, column) = size()?;
        Ok(RenderMgr {
            rows: row,
            columns: column,
            stdout: stdout(),
            draw_data: None
        })
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
    pub fn draw(&mut self, cfg: &Config) -> Result<()> {

        self.stdout.execute(terminal::Clear(terminal::ClearType::All))?;
        self.stdout.execute(style::SetBackgroundColor(cfg.default_background_color))?;
        // draw_header
        self.draw_panel("header", cfg, true)?;

        if let Some(x)= &self.draw_data {
            // draw document data
            self.draw_doc(cfg)?;
        } else {
            self.draw_splash_screen()?;
        }
        // draw footer
        self.draw_panel("footer", cfg, false)?;

        self.draw_debug_data()?;
        Ok(())
    }
    // Не работает
    fn draw_panel(&mut self, label: &str, cfg: &Config, is_header: bool) -> Result<()> {
        let to_draw = " ".repeat(self.rows as usize - label.len()) + label;

        self.write_on_bg(to_draw, cfg)?;

        for _ in 0..if is_header {cfg.header_height} else {cfg.footer_height} - 1 {
            self.write_on_bg(" ".repeat(self.rows as usize), cfg)?;
        }
        Ok(())
    }

    fn draw_doc(&mut self, cfg: &Config) -> Result<()> {
        /*
        let cols_remains = self.columns
            - cfg.header_height
            - cfg.footer_height;
            - ();

        for i in 0..cols_remains {
            let i_s = i.to_string();
            let to_write = (" ".repeat(cfg.padding_size as usize - &i_s.len())+ &i_s)
                + " ";
            self.write_text(to_write, cfg)?;
        }
        */
        let mut line = 1;
        for data in &self.draw_data.unwrap() {
            let line_str = line.to_string();
            let to_write = " ".repeat(cfg.padding_size as usize - line_str.len())
                                        + &line_str + data;
            line += 1;
            self.write_text(to_write, cfg)?;
        }

        // Если осталось место дорисовать столько строк, сколько необходимо
        Ok(())
    }

    fn draw_splash_screen(&mut self) -> Result<()> {


        Ok(())
    }

    fn write_on_bg(&mut self, data: String, cfg: &Config) -> Result<()> {
        write!(self.stdout, "\r{}\n", data
            .on(cfg.default_background_color)
            .with(cfg.default_font_color)
        )?;
        Ok(())
    }

    fn write_text(&mut self, data: String, cfg: &Config) -> Result<()> {
        write!(self.stdout, "\r{}\n", data
            .with(cfg.default_font_color)
        )?;
        Ok(())
    }

    fn draw_debug_data(&mut self) -> Result<()>{
        write!(self.stdout, "\r{:?}\n", &self.draw_data)?;
        Ok(())
    }
}