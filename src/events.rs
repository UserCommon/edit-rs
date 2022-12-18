use crate::{utils::{Direction, Events::{self, *}}, terminal::Terminal};
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


pub struct EventQueue(Vec<Events>);

impl EventQueue {
    pub fn new() -> Self {
        EventQueue(vec![])
    }

    pub fn get(&self) -> &Vec<Events> {
        &self.0
    }

    pub fn push(&mut self, item: Events) {
        self.0.insert(0, item);
    }

    pub fn remove(&mut self) {
        self.0.pop().unwrap();
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}

pub struct EventMgr {
    pub queue: EventQueue
}


impl EventMgr {
    pub fn new() -> Self {
        EventMgr {
            queue: EventQueue::new()
        }
    }

    pub fn event_manager(&mut self) -> Result<()> {
        match read()? {
            Event::Key(key_event) => self.key_manager(key_event),
            Event::Resize(columns, rows) => self.queue.push(Resize(columns, rows)),
            _ => ()
        }

        Ok(())
    }

    fn key_manager(&mut self, key_event: KeyEvent) {
        match key_event {

            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => self.queue.push(Quit),

            KeyEvent {
                code: KeyCode::Up,
            ..
            } => self.queue.push(MoveUp),


            KeyEvent {
                code: KeyCode::Down,
                ..
            } => self.queue.push(MoveDown),


            KeyEvent {
                code: KeyCode::Left,
                ..
            } => self.queue.push(MoveLeft),

            KeyEvent {
                code: KeyCode::Right,
                ..
            } => self.queue.push(MoveRight),

            KeyEvent {
                code: KeyCode::Backspace,
                ..
            } => self.queue.push(Erase),

            KeyEvent {
                code: KeyCode::Char(' '),
                ..
            } => self.queue.push(Space),

            KeyEvent {
                code: KeyCode::Char(ch),
                ..
            } => self.queue.push(Write(ch)),
            

            _ => ()
        }

    }
}
