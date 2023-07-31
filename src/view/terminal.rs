use std::fmt::Display;
use std::io::{self, Write};

use crossterm::cursor::{self, SetCursorStyle};
use crossterm::event::{self, EnableMouseCapture, KeyEvent, MouseEventKind};
use crossterm::{terminal as cterminal, Command};

use super::Position;
use crate::error::NanoResult;

pub type TerminalSize = (u16, u16);

/// Terminal view
///
/// This struct is used to store the terminal view state.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct TerminalView {
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

impl Display for TerminalView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(width: {}, height: {}, scroll_offset: {}, cursor_position: {})",
            self.width, self.height, self.offset, self.cursor
        )
    }
}

impl TerminalView {
    /// Create a new terminal view
    ///
    /// The terminal view is initialized with the current terminal size.
    pub fn new() -> NanoResult<Self> {
        TerminalView::init()?;
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
        TerminalView::execute(cterminal::EnterAlternateScreen)?;
        cterminal::enable_raw_mode()?;
        TerminalView::set_cursor_style(SetCursorStyle::BlinkingBar)?;
        TerminalView::execute(EnableMouseCapture)?;

        Ok(())
    }

    /// Reset the terminal view
    /// This will disable raw mode and leave the alternate screen
    /// It will also disable mouse capture
    /// This should be called before exiting the program
    pub fn reset() -> NanoResult<()> {
        TerminalView::execute(cterminal::LeaveAlternateScreen)?;
        TerminalView::set_cursor_style(SetCursorStyle::SteadyBar)?;
        TerminalView::execute(event::DisableMouseCapture)?;
        TerminalView::show_cursor()?;
        cterminal::disable_raw_mode()?;

        Ok(())
    }

    /// Get the current terminal view
    pub fn size(&self) -> TerminalSize {
        (self.width, self.height)
    }

    /// Set the title of the terminal
    pub fn set_title<S>(title: S) -> NanoResult<()>
    where
        S: AsRef<str> + Display,
    {
        TerminalView::execute(cterminal::SetTitle(title))
    }

    /// Set the cursor position
    pub fn set_cursor_position(&mut self, cursor: Position) {
        self.cursor = cursor;

        let (x, y) = (cursor.x, cursor.y);

        TerminalView::execute(cursor::MoveTo(x, y)).expect("Failed to move cursor");
    }

    pub fn set_cursor_style(cursor_style: SetCursorStyle) -> NanoResult<()> {
        TerminalView::execute(cursor_style)?;
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
        TerminalView::execute(cursor::Hide)
    }

    /// Show the cursor
    pub fn show_cursor() -> NanoResult<()> {
        TerminalView::execute(cursor::Show)
    }

    /// Clear the current line
    pub fn clear_current_line() -> NanoResult<()> {
        TerminalView::execute(cterminal::Clear(cterminal::ClearType::CurrentLine))
    }

    /// Write a string to the terminal
    pub fn write<S: AsRef<str>>(s: S) {
        println!("{}\r", s.as_ref());
    }

    /// Clears the terminal
    pub fn clear() -> NanoResult<()> {
        TerminalView::execute(cterminal::Clear(cterminal::ClearType::All))
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
