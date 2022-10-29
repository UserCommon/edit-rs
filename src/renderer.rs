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
use crossterm::style::{SetBackgroundColor, SetForegroundColor, style};
use crate::Cursor;
use crate::terminal::Terminal;


pub const RESET_BG: SetBackgroundColor = SetBackgroundColor(Color::Reset);
pub const RESET_FG: SetForegroundColor = SetForegroundColor(Color::Reset);

pub enum KindOfPanel {
    Header,
    Footer
}

// TODO! Refactor & add config field
pub struct DataStorage {
    data: String,
}

impl DataStorage {
    pub fn new() -> Self {
        DataStorage {
            data: String::from("\r")
        }
    }

    pub fn append_row(&mut self, row: String) {
        self.data += &format!("\r{}\n", row);
    }

    pub fn append_data(&mut self, data: String) {
        self.data += &format!("\r{}", data);
    }

    pub fn get_data(&self) -> &String {
        &self.data
    }

    pub fn get_data_by_rows(&self) -> Vec<String> {
        self.data.split("\n")
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    }

    pub fn clear(&mut self) {
        self.data = String::from("\r");
    }

}


pub struct RenderMgr {
    pub terminal: Terminal,
    stdout: Stdout,
    data: DataStorage,
    raw_data: Vec<String>,
    cfg: Option<Config>
}

impl RenderMgr {
    pub fn new() -> Self {
        RenderMgr {
            terminal: Terminal::new().unwrap(),
            stdout: stdout(),
            data: DataStorage::new(),
            raw_data: Vec::new(),
            cfg: None,
        }
    }

    pub fn set_config(&mut self, cfg: Config) {
        self.cfg = Some(cfg);
    }

    pub fn set_raw_data(&mut self, raw_data: Vec<String>) {
        self.raw_data = raw_data;
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
    // ПЕРЕДАТЬ ВЛАДЕНИЕ КУРСОРОМ ТЕРМИНАЛУ
    pub fn update(&mut self) -> Result<()> {
        let pos = self.terminal.cursor.get_pos().clone();
        Terminal::clear();
        Terminal::hide_cursor();
        execute!(&self.stdout,
            MoveTo(0, 0)
        )?;

        self.form_data();
        write!(&self.stdout, "{}", self.data.get_data())?;

        execute!(&self.stdout,
            MoveTo(pos.0, pos.1),
            SetCursorShape(self.terminal.cursor.shape()),
        )?;

        
        self.data.clear();
        Terminal::show_cursor();
        Terminal::flush();
        Ok(())
    }

    pub fn form_data(&mut self) {
        let pos = self.terminal.cursor.get_pos();
        self.append_panel(
            &format!(
                "Cursor: X: {}, Y {}",
                pos.0,
                pos.1
            ),
            KindOfPanel::Header
        );

        self.append_doc();

        // update footer
        self.append_panel(
            &format!("Document: splash.txt"),
            KindOfPanel::Footer
        );
    }

    // Не работает
    fn append_panel(&mut self, label: &str, kind_of_panel: KindOfPanel) {
        let cfg = self.get_cfg();
        match kind_of_panel {
            KindOfPanel::Footer | KindOfPanel::Header => self.append_header_or_footer(label, &cfg.clone()),

        }
    }

    fn append_header_or_footer(&mut self, label: &str, cfg: &Config) {

        let mut to_draw = format!(
            "{}{}{}",
            Config::rgb_bg(cfg.default_background_color),
            " ".repeat(self.terminal.rows as usize - label.len()) + label,
            RESET_BG
        );

        for _ in 0..cfg.header_height - 1 {
            let row = format!(
                "{}{}{}",
                Config::rgb_bg(cfg.default_background_color),
                " ".repeat(self.terminal.rows as usize),
                RESET_BG
            );
            to_draw += &row;
        }

        self.data.append_data(to_draw);
    }

    fn append_doc(&mut self) {
        let cfg = &self.get_cfg().clone();
        let mut line = 1;

        self.data.append_row("".to_string());
        for row in &self.raw_data {
            let doc_row = format!(
                "{}{}{}",
                Config::rgb_fg(cfg.default_font_color),
                row,
                RESET_FG,

            );
            let to_write = format!("{} {}", self.str_enumeration(line), doc_row);
            self.data.append_row(to_write);
            line += 1;
        }

        // ВОТ ЭТУ ХУЙНЮ УБРАТЬ ПАЛЮБАСУ В ПРОСТО РЕНДЕР
        // А ТО БУДЕТ КАПОШИТЬ НАХУЙ ПРИ СЕЙВЕ
        if line < self.terminal.columns {
            for _ in 0..self.terminal.columns - line - 1 {
                let to_write = self.str_enumeration(line);
                self.data.append_row(to_write);
                line += 1;
            }
        }
        // Если осталось место дорисовать столько строк, сколько необходимо

    }

    fn str_enumeration(&self, num: u16) -> String {
        let cfg = self.get_cfg();
        let str_num = num.to_string();
        let to_draw = format!(
            "{}{}{}{}{}{}",
            Config::rgb_bg(cfg.default_background_color),
            " ".repeat(cfg.padding_size as usize - str_num.len()),
            Config::rgb_fg(cfg.default_font_color),
            str_num,
            RESET_FG,
            RESET_BG,
        );

        to_draw
    }

    fn draw_splash_screen(&mut self)  {
        ()
    }



    fn get_cfg(&self) -> &Config {
        self.cfg.as_ref().unwrap()
    }


}