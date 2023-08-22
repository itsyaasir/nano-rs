use std::fmt::Display;
use std::io::{self, Write};

use crossterm::cursor::{self, SetCursorStyle};
use crossterm::event::{self, EnableMouseCapture, KeyEvent};
use crossterm::{terminal as cterminal, Command};

use crate::error::NanoResult;

/// Cursor position
#[derive(Debug, Clone, Copy, Default)]
pub struct Position {
    /// The x position of the cursor
    pub x: u16,
    /// The y position of the cursor
    pub y: u16,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

impl From<(u16, u16)> for Position {
    fn from((x, y): (u16, u16)) -> Self {
        Position { x, y }
    }
}

/// Terminal view
///
/// This struct is used to store the terminal view state.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Terminal {
    /// The width of the terminal view
    pub width: u16,

    /// The height of the terminal view
    pub height: u16,

    /// The current scroll offset
    pub offset: Position,

    /// The current cursor position, relative to the terminal view
    /// It is a tuple of (x, y) - (column, row)
    pub cursor: Position,
}

impl Display for Terminal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(width: {}, height: {}, scroll_offset: {}, cursor_position: {})",
            self.width, self.height, self.offset, self.cursor
        )
    }
}

impl Terminal {
    /// Create a new terminal view
    ///
    /// The terminal view is initialized with the current terminal size.
    pub fn new() -> NanoResult<Self> {
        Terminal::init()?;
        let (width, height) = cterminal::size()?;
        Ok(Self {
            width,
            height: height - 2, // Subtract 2 for the status bar

            offset: Position::default(),
            cursor: Position::default(),
        })
    }

    /// Initialize the terminal view
    /// This will enable raw mode and enter the alternate screen
    /// It will also enable mouse capture
    /// This should be called before starting the program
    ///
    pub fn init() -> NanoResult<()> {
        Terminal::execute(cterminal::EnterAlternateScreen)?;
        cterminal::enable_raw_mode()?;
        Terminal::set_cursor_style(SetCursorStyle::BlinkingBar)?;
        Terminal::execute(EnableMouseCapture)?;

        Ok(())
    }

    /// Reset the terminal view
    /// This will disable raw mode and leave the alternate screen
    /// It will also disable mouse capture
    /// This should be called before exiting the program
    pub fn reset() -> NanoResult<()> {
        Terminal::execute(cterminal::LeaveAlternateScreen)?;
        Terminal::set_cursor_style(SetCursorStyle::SteadyBar)?;
        Terminal::execute(event::DisableMouseCapture)?;
        Terminal::show_cursor()?;
        cterminal::disable_raw_mode()?;

        Ok(())
    }

    /// Set the title of the terminal
    pub fn set_title<S>(title: S) -> NanoResult<()>
    where
        S: AsRef<str> + Display,
    {
        Terminal::execute(cterminal::SetTitle(title))
    }

    /// Set the cursor position
    pub fn set_cursor_position(&mut self, cursor: Position) {
        self.cursor = cursor;

        let (x, y) = (cursor.x, cursor.y);

        Terminal::execute(cursor::MoveTo(x, y)).expect("Failed to move cursor");
    }

    pub fn set_cursor_style(cursor_style: SetCursorStyle) -> NanoResult<()> {
        Terminal::execute(cursor_style)?;
        Ok(())
    }

    /// Flush the terminal
    /// This is used to flush the terminal buffer
    ///
    /// Basically, this clears the terminal buffer
    pub fn flush() -> NanoResult<()> {
        io::stdout().flush()?;
        Ok(())
    }

    /// Execute a crossterm command
    ///
    pub fn execute<C: Command>(command: C) -> NanoResult<()> {
        crossterm::execute!(std::io::stdout(), command)?;

        Ok(())
    }

    /// Hide the cursor
    pub fn hide_cursor() -> NanoResult<()> {
        Terminal::execute(cursor::Hide)
    }

    /// Show the cursor
    pub fn show_cursor() -> NanoResult<()> {
        Terminal::execute(cursor::Show)
    }

    /// Clear the current line
    pub fn clear_current_line() -> NanoResult<()> {
        Terminal::execute(cterminal::Clear(cterminal::ClearType::CurrentLine))
    }

    /// Write a string to the terminal
    pub fn write<S: AsRef<str>>(s: S) {
        println!("{}\r", s.as_ref());
    }

    /// Clears the terminal
    pub fn clear() -> NanoResult<()> {
        Terminal::execute(cterminal::Clear(cterminal::ClearType::All))
    }

    /// Read a key from the terminal
    pub fn read_key(&mut self) -> NanoResult<KeyEvent> {
        loop {
            match event::read()? {
                event::Event::Key(event) => return Ok(event),
                _ => continue,
            }
        }
    }
}
