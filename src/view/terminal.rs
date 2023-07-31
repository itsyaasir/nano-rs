use std::fmt::Display;
use std::io::{self, Write};

use crossterm::cursor::{self, SetCursorStyle};
use crossterm::event::{self, EnableMouseCapture};
use crossterm::{style, terminal as cterminal, Command};

use crate::error::NanoResult;

pub type TerminalSize = (u16, u16);

/// Terminal view
///
/// This struct is used to store the terminal view state.
#[derive(Debug, Clone, Copy)]
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
            "(width: {}, height: {}, scroll_offset: {}, cursor_position: {:?})",
            self.width, self.height, self.offset, self.cursor
        )
    }
}

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
impl TerminalView {
    /// Create a new terminal view
    ///
    /// The terminal view is initialized with the current terminal size.
    pub fn new() -> NanoResult<Self> {
        TerminalView::init()?;
        let (width, height) = cterminal::size()?;
        Ok(Self {
            width,
            height,
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
        cterminal::enable_raw_mode()?;
        TerminalView::set_cursor_style(SetCursorStyle::BlinkingBlock)?;
        TerminalView::execute(EnableMouseCapture)?;

        Ok(())
    }

    /// Reset the terminal view
    /// This will disable raw mode and leave the alternate screen
    /// It will also disable mouse capture
    /// This should be called before exiting the program
    pub fn reset() -> NanoResult<()> {
        cterminal::disable_raw_mode()?;
        TerminalView::execute(cterminal::LeaveAlternateScreen)?;
        TerminalView::execute(event::DisableMouseCapture)?;
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
    pub fn write<S: AsRef<str>>(s: S) -> NanoResult<()> {
        TerminalView::execute(cterminal::Clear(cterminal::ClearType::CurrentLine))?;

        print!("{}", s.as_ref());
        Ok(())
    }

    /// Sets the foreground color
    pub fn set_fg_color(color: syntect::highlighting::Color) -> NanoResult<()> {
        TerminalView::execute(style::SetForegroundColor(style::Color::Rgb {
            r: color.r,
            g: color.g,
            b: color.b,
        }))
    }

    /// Clears the terminal
    pub fn clear() -> NanoResult<()> {
        TerminalView::execute(cterminal::Clear(cterminal::ClearType::All))
    }
}
