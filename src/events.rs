use crate::utils::Direction;
use std::collections::vec_deque::VecDeque;
use std::time::Duration;
use std::io::Result;

use crossterm::event::{
    Event,
    poll,
    read,
};

pub struct EventMgr {
    queue: VecDeque<T>
}

impl EventMgr {
    pub fn new() -> Self {
        EventMgr {
            queue: VecDeque::new()
        }
    }

    pub fn event_manager(&mut self) -> Result<()> {
        if poll(Duration::from_millis(500)) {
            match read()? {
                Event::Key(char) => self.queue.push_back(char),
                _ => ()
            }
        }
        Ok(())
    }
}