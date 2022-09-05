use crate::config::Config;

use std::io::{Stdout, stdout, Write, Result};

use crossterm::{
    execute,
    terminal::{
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

pub struct RenderMgr<'a> {
    rows: u16,
    columns: u16,
    cfg: &'a Config,
    stdout: Stdout
}

impl RenderMgr {
    pub fn new(cfg: &Config) -> Self {
        let (column, row) = size()?;
        RenderMgr {
            rows: row,
            columns: column,
            stdout: stdout(),
            cfg
        }
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
    pub fn draw(&mut self) -> Result<()> {
        // draw_header
        self.draw_panel("label")?;
        for _ in self.cfg.header_height {
            self.draw_panel("")?;
        }

        // draw document data


        // draw footer
        self.draw_panel("label")?;
        for _ in self.cfg.footer_height {
            self.draw_panel("")?;
        }
        Ok(())
    }

    fn draw_panel(&mut self, label: &str) -> Result<()> {
        let to_draw = " ".repeat((self.rows - label.len()) as usize) + label;
        write!(self.stdout, "\r{}", to_draw
                                        .on(self.cfg.default_background_color)
                                        .with(self.cfg.default_font_color))?;
        Ok(())
    }

    fn draw_doc(&mut self, data: String) -> Result<()> {
        let cols_remains = self.columns
                                - self.cfg.header_height
                                - self.cfg.footer_height
                                - data.chars().count('\n');
        write!(self.stdout, "\r{}", data)?;

    }
}