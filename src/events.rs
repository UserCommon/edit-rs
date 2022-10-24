use crate::utils::{Direction, Events::{self, *}};
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

#[derive(Debug)]
pub struct EventStack {
    history: Vec<Vec<Events>>,
    curr_patch: Vec<Events>
}

impl EventStack {
    pub fn new() -> Self {
        EventStack {
            history: vec![],
            curr_patch: vec![]
        }
    }

    pub fn push(&mut self, event: Events) {
        self.curr_patch.insert(0, event);
    }

    pub fn pop(&mut self) -> Option<Vec<Events>> {
        self.history.pop()
    }

    pub fn append(&mut self, events: Vec<Events>) {
        self.history.push(events);
    }

    pub fn clear(&mut self) {
        self.history.clear();
    }

    pub fn commit(&mut self) {
        if !self.curr_patch.is_empty() {
            self.history.push(self.curr_patch.clone());
            self.curr_patch.clear();
        }
    }

    pub fn len(&self) -> usize {
        self.history.len()
    }

    pub fn get_events(&self) -> &Vec<Events> {
        &self.curr_patch
    }
}

pub struct EventMgr {
    pub queue: EventStack
}

impl EventMgr {
    pub fn new() -> Self {
        EventMgr {
            queue: EventStack::new()
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
                modifiers: CONTROL,
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
                code: KeyCode::Char(ch),
                ..
            } => self.queue.push(Write(ch)),

            _ => ()
        }

    }
}