use crate::{
    utils,
    cursor::Cursor,
    events,
    renderer,
    config::Config
};

pub struct Programm {
    pub cfg: Config,
    cursor: Cursor
}