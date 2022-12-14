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
use crate::cursor::*;
use crate::terminal::Terminal;


pub const RESET_BG: SetBackgroundColor = SetBackgroundColor(Color::Reset);
pub const RESET_FG: SetForegroundColor = SetForegroundColor(Color::Reset);

pub enum KindOfPanel {
    Header,
    Footer
}

// TODO! Refactor & add config field
pub struct DataStorage {
    pub data: Vec<Vec<char>>,
}

pub trait DataReadMethods {
    fn get_data(&self) -> &Vec<Vec<char>>;
    fn get_data_row(&self, indecies: usize) -> &Vec<char>;
}

pub trait DataModifyMethods {
    fn append_data_row(&mut self, row: String);
    fn append_data(&mut self, data: String);
    fn clear(&mut self);
    
    fn edit_line(&mut self, index: usize, line: String);
}

impl DataStorage {
    pub fn new() -> Self {
        DataStorage {
            data: vec![vec!['\r']],
        }
    }

}

impl DataModifyMethods for DataStorage {
    fn append_data_row(&mut self, row: String) {
        let row = &format!("\r{}\n", row);
        self.data.push(row.chars().collect());
    }

    fn append_data(&mut self, data: String) {
        self.data.push(data.chars().collect()); 
    }

   
    fn clear(&mut self) {
        self.data = vec![vec!['\n']];
    }

    fn edit_line(&mut self, index: usize, line: String) {
        self.data[index] = line.chars().collect();
    }
}

impl DataReadMethods for DataStorage {
    fn get_data(&self) -> &Vec<Vec<char>> {
        &self.data
    }

    fn get_data_row(&self, index: usize) -> &Vec<char> {
        &self.data[index]
    }
}

pub struct RenderMgr {
    pub terminal: Terminal,
    stdout: Stdout,
    pub data: DataStorage,
    //pub raw_data: Vec<String>,
    cfg: Option<Config>
}

impl RenderMgr {
    pub fn new() -> Self {
        RenderMgr {
            terminal: Terminal::new().unwrap(),
            stdout: stdout(),
            data: DataStorage::new(),
            //raw_data: Vec::new(),
            cfg: None,
        }
    }

    pub fn set_config(&mut self, cfg: Config) {
        self.cfg = Some(cfg);
        //self.terminal.set_config(cfg);
    }

    /*
    pub fn set_raw_data(&mut self, raw_data: Vec<String>) {
        self.raw_data = raw_data;
    }
    */

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
    pub fn update(&mut self) -> Result<()> {
        let pos = self.terminal.cursor.get_pos().clone();
        
        Terminal::hide_cursor();
        execute!(&self.stdout,
            MoveTo(0, 0)
        )?;

        //Terminal::clear(); // <= Too slow

        self.form_data();

        let mut string = "".to_string();
        let data = self.data.get_data();
        for col in data {
            for row in col {
                string.push(*row);
            }
        } 
        write!(&self.stdout, "{}", string)?;

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

    // ???? ????????????????
    fn append_panel(&mut self, label: &str, kind_of_panel: KindOfPanel) {
        let cfg = self.get_cfg();
        match kind_of_panel {
            KindOfPanel::Footer | KindOfPanel::Header => self.append_header_or_footer(label, &cfg.clone()),

        }
    }

    fn append_header_or_footer(&mut self, label: &str, cfg: &Config) {

        let mut to_draw = format!(
            "\r{}{}{}",
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

        self.data.append_data_row("".to_string());
        for row in self.data.get_data() {
            let doc_row = format!(
                "{}{}{}",
                Config::rgb_fg(cfg.default_font_color),
                row.iter().collect::<String>(),
                RESET_FG,

            );
            let to_write = format!("{} {}", self.str_enumeration(line), doc_row);
            self.data.append_data_row(to_write);
            line += 1;
        }

        // ?????? ?????? ?????????? ???????????? ???????????????? ?? ???????????? ????????????
        // ?? ???? ?????????? ???????????????? ?????????? ?????? ??????????
        if line < self.terminal.columns {
            for _ in 0..self.terminal.columns - line - 1 {
                let to_write = self.str_enumeration(line);
                self.data.append_data_row(to_write);
                line += 1;
            }
        }
        // ???????? ???????????????? ?????????? ???????????????????? ?????????????? ??????????, ?????????????? ????????????????????

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

impl DataReadMethods for RenderMgr {
    fn get_data(&self) -> &Vec<Vec<char>> {
        self.data.get_data()
    }

    fn get_data_row(&self, indecies: usize) -> &Vec<char> {
        self.data.get_data_row(indecies)
    }
}
