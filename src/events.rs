use crate::utils::{Direction, Todo::{self, *}};
use std::collections::vec_deque::VecDeque;
use std::time::Duration;
use std::io::Result;

use crossterm::event::{
    KeyModifiers,
    KeyEvent,
    KeyCode,
    Event,
    poll,
    read,
};

pub struct EventMgr {
    pub queue: VecDeque<Todo>
}

impl EventMgr {
    pub fn new() -> Self {
        EventMgr {
            queue: VecDeque::new()
        }
    }

    pub fn event_manager(&mut self) -> Result<()> {
        if poll(Duration::from_millis(500))? {
            match read()? {
                Event::Key(key_event) => self.key_manager(key_event),
                Event::Resize(columns, rows) => self.queue.push_back(Resize(columns, rows)),

                _ => ()
            }
        }
        Ok(())
    }

    fn key_manager(&mut self, key_event: KeyEvent) {
        match key_event {

            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: CONTROL,
                ..
            } => self.queue.push_back(Quit),

            KeyEvent {
                code: KeyCode::Up,
            ..
            } => self.queue.push_back(MoveUp),


            KeyEvent {
                code: KeyCode::Down,
                ..
            } => self.queue.push_back(MoveDown),


            KeyEvent {
                code: KeyCode::Left,
                ..
            } => self.queue.push_back(MoveLeft),

            KeyEvent {
                code: KeyCode::Right,
                ..
            } => self.queue.push_back(MoveRight),

            KeyEvent {
                code: KeyCode::Char(ch),
                ..
            } => self.queue.push_back(Write(ch)),

            _ => ()
        }

    }
}