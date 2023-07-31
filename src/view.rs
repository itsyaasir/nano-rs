use std::fmt::Display;

pub mod editor;
pub mod terminal;

pub use editor::NanoEditor;
pub use terminal::{TerminalSize, TerminalView};

/// Cursor position
#[derive(Debug, Clone, Copy, Default)]
pub struct Position {
    /// The x position of the cursor
    pub x: u16,
    /// The y position of the cursor
    pub y: u16,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

impl From<(u16, u16)> for Position {
    fn from((x, y): (u16, u16)) -> Self {
        Position { x, y }
    }
}
