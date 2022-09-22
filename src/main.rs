mod cursor;
mod renderer;
mod events;
mod config;
mod program;
mod utils;
mod files;

use std::io::Result;

use utils::{Direction, Todo};
use config::Config;
use program::Program;
use cursor::Cursor;



fn main() -> Result<()>{
    let mut app = Program::builder()
                .build();
    app.file.add_file("to_read.txt")?;
    app.run()?;
    Ok(())
}
