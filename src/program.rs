use crate::{
    utils,
    cursor::Cursor,
    events,
    renderer,
    config::Config
};

pub struct Program {
    pub cfg: Config,
    cursor: Cursor
}