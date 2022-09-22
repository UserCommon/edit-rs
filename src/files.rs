use std::fs::File;
use std::io::{Read, Result};
use std::ops::Range;


pub struct FileMgr {
    files: Vec<File>,
    curr: usize
}

impl FileMgr {
    pub fn new() -> Self {
        FileMgr {
            files: vec![],
            curr: 0
        }
    }

    pub fn add_file(&mut self, path: &str) -> Result<()> {
        let mut file = File::options()
            .append(true)
            .read(true)
            .open(path)?;

        if !self.files.is_empty() { self.curr += 1; }
        self.files.push(file);

        Ok(())
    }

    pub fn close_file(&mut self) {
        // probably contains bug
        self.files.remove(self.curr);
        if self.curr > 0 { self.curr -= 1};
    }

    pub fn get_text(&mut self) -> Vec<String> {
        let mut data_string = String::new();
        self.files[self.curr].read_to_string(&mut data_string).expect("can't read file");

        data_string.lines().map(|x| {x.to_string()}).collect::<Vec<String>>()
    }
}