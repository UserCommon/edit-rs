use crate::config::Config;

use std::io::{Stdout, stdout, Write, Result};
use std::ptr::write;

use crossterm::{
    execute,
    terminal::{
        self,
        size,
        EnterAlternateScreen,
        LeaveAlternateScreen,
        enable_raw_mode,
        disable_raw_mode
    }, style::{
        Stylize,
        Color,
    },
    ExecutableCommand
};


pub struct RenderMgr {
    rows: u16,
    columns: u16,
    stdout: Stdout,
}

impl RenderMgr {
    pub fn new() -> Result<Self> {
        let (row, column) = size()?;
        Ok(RenderMgr {
            rows: row,
            columns: column,
            stdout: stdout(),
        })
    }

    pub fn set_size(&mut self, size: (u16, u16)) {
        self.rows = size.0;
        self.columns = size.1;
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
        // draw_header
        self.draw_panel("label", cfg, true)?;

        // draw document data
        self.draw_doc("yep".to_string(), cfg)?;

        // draw footer
        self.draw_panel("label", cfg, false)?;
        Ok(())
    }
    // Не работает
    fn draw_panel(&mut self, label: &str, cfg: &Config, is_header: bool) -> Result<()> {
        let to_draw = " ".repeat(self.rows as usize - label.len()) + label;
        for _ in 0..if is_header {cfg.header_height} else {cfg.footer_height} - 1 {
            write!(self.stdout, "\r{}", to_draw
                .on(cfg.default_background_color)
                .with(cfg.default_font_color))?;
        }
        Ok(())
    }

    fn draw_doc(&mut self, data: String, cfg: &Config) -> Result<()> {
        let cols_remains = self.columns
                                - cfg.header_height
                                - cfg.footer_height
                                - (data.lines().count() as u16);
        write!(self.stdout, "\r{}", data)?;
        Ok(())
    }
}