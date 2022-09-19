mod cursor;
mod renderer;
mod events;
mod config;
mod program;
mod utils;
mod files;

use utils::{Direction, Todo};
use config::Config;
use program::Program;
use cursor::Cursor;

fn main() {
    let mut app = Program::builder()
                .build();
    app.run().unwrap();
}
