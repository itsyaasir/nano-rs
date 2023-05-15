use std::error::Error;

use crossterm::event::{read, Event, KeyCode};

#[derive(Clone, Debug)]
pub struct NanoArgs {
    pub file: Option<String>,
}

impl NanoArgs {
    pub fn new() -> Self {
        Self { file: None }
    }
}
#[derive(Debug)]
pub struct Editor {
    cursor: Cursor,
    buffer: Vec<String>,
}

#[derive(Debug)]
pub struct Cursor {
    x: usize,
    y: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = NanoArgs::parse();
    match read()? {
        Event::Key(event) => match event.code {
            KeyCode::Char('q') => {
                // Quit the program
                println!("Quit");

                std::process::exit(0);
            }
            _ => Ok(()),
        },
        _ => Ok(()),
    }
}
