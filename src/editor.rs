use std::env;
use std::path::PathBuf;
use std::str::FromStr;

use crossterm::event::KeyCode;
use crossterm::style::{Color, Stylize};
use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;

use crate::buffer::Buffer;
use crate::config::configuration::NanoConfiguration;
use crate::content::Data;
use crate::error::{NanoError, NanoResult};
use crate::terminal::{Position, Terminal};

pub const NANO_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The Nano editor
///
/// This is the main editor struct. It contains the terminal view, the file
/// being edited, and the cursor position.
/// It also contains the offset of the view, which is used to scroll the view.
///
#[derive(Debug)]
pub struct NanoEditor {
    /// The terminal view
    terminal: Terminal,
    buffer: Buffer,
    config: NanoConfiguration,
}

impl NanoEditor {
    /// Create a new Nano editor
    /// This will create a new editor instance and initialize the terminal
    /// view, file, and cursor.
    ///
    /// # Errors
    /// This function will return an error if the terminal view cannot be
    /// initialized.
    ///
    /// # Examples
    /// ```
    /// use nano::NanoEditor;
    /// let mut editor = NanoEditor::new().unwrap();
    /// ```
    ///
    pub fn new(config: NanoConfiguration) -> NanoResult<Self> {
        let args = env::args().collect::<Vec<String>>();
        let file_name =
            PathBuf::from_str(&args[1]).map_err(|e| NanoError::FileError(e.to_string()))?;
        let file = Buffer::from_file(file_name)?;
        let terminal_view = Terminal::new()?;

        Ok(Self {
            terminal: terminal_view,
            buffer: file,
            config,
        })
    }

    /// The main loop of the editor
    /// This will run the main loop of the editor, which will render the editor
    /// and handle events.
    ///
    /// # Errors
    /// This function will return an error if the editor cannot be rendered.
    pub fn run(&mut self) -> NanoResult<()> {
        Terminal::set_title(&format!(
            "Nano - {}",
            self.buffer
                .name
                .as_ref()
                .unwrap_or(&String::from("Untitled"))
        ))?;

        loop {
            if let Err(e) = self.render() {
                NanoEditor::handle_error(e)?;
            }

            if let Err(e) = self.process_key() {
                NanoEditor::handle_error(e)?;
            }
        }
    }

    pub fn draw_status_bar(&mut self) -> NanoResult<()> {
        let status_bar_message = format!(
            "Nano {} - File: {} Modified", // TODO::Change Modified to showcase if the file is dirty or not.
            NANO_VERSION,
            self.buffer
                .name
                .as_ref()
                .unwrap_or(&String::from("Untitled")),
        )
        .with(Color::Black)
        .on(Color::White)
        .to_string();

        // Get the terminal width and the text length
        let terminal_width = self.terminal.width;
        let text_length = status_bar_message.len();

        // Calculate the number of spaces to add on each side of the text
        let num_spaces = (terminal_width as usize - text_length) / 2;
        let centered_text = format!(
            "{:>width$}",
            status_bar_message,
            width = text_length + num_spaces
        );

        Terminal::write(centered_text);

        Ok(())
    }

    /// Process the key event captured from the terminal
    pub fn process_key(&mut self) -> NanoResult<()> {
        let event = self.terminal.read_key()?;

        match event.code {
            KeyCode::Char('q') => NanoEditor::exit()?,
            KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down => {
                self.navigate_cursor(event.code)
            }
            _ => {}
        }

        Ok(())
    }

    fn navigate_cursor(&mut self, event: KeyCode) {
        let Position { mut x, mut y } = self.terminal.cursor;
        let document_height = self.buffer.len() as u16;
        let document_width = self
            .buffer
            .row(y as usize)
            .map_or(0, |content| content.len()) as u16;

        if x > document_width {
            x = document_width
        }

        match event {
            KeyCode::Down => {
                if y > document_height {
                    y = document_height
                }
                y = y.saturating_add(1)
            }
            KeyCode::Up => y = y.saturating_sub(1),
            KeyCode::Left => x = x.saturating_sub(1),
            KeyCode::Right => x = x.saturating_add(1),
            _ => {}
        };

        self.terminal.set_cursor_position((x, y).into())
    }

    /// Render the editor
    /// This will render the editor, including the file, cursor, and status bar.
    ///
    fn render(&mut self) -> NanoResult<()> {
        Terminal::hide_cursor()?;

        self.draw_status_bar()?;
        self.render_contents()?;

        self.terminal.set_cursor_position(Position {
            x: self
                .terminal
                .cursor
                .x
                .saturating_sub(self.terminal.offset.x),
            y: self
                .terminal
                .cursor
                .y
                .saturating_sub(self.terminal.offset.y),
        });

        Terminal::show_cursor()?;
        Terminal::flush()?;
        Ok(())
    }

    fn render_contents(&mut self) -> NanoResult<()> {
        let height = self.terminal.height;

        for terminal_row in 0..height {
            Terminal::clear_current_line()?;

            if let Some(content) = self
                .buffer
                .row(terminal_row as usize + self.terminal.offset.y as usize)
            {
                self.render_content(content, terminal_row)?
            } else {
                Terminal::write("~\r");
            }
        }

        Ok(())
    }

    fn render_content(&self, content: &Data, _line_number: u16) -> NanoResult<()> {
        let width = self.terminal.width as usize;
        let start = self.terminal.offset.x as usize;
        let end = self.terminal.offset.x as usize + width;
        let text = &content.display_range(start, end);

        let ss = SyntaxSet::load_defaults_newlines();
        let syntax =
            ss.find_syntax_by_extension(self.buffer.file_type())
                .ok_or(NanoError::Generic(format!(
                    "Syntax not found for {}",
                    self.buffer.file_type()
                )))?;

        let theme = &self.config.load_themes().expect("failed to load theme");

        let mut h = HighlightLines::new(syntax, theme);

        let ranges: Vec<(syntect::highlighting::Style, &str)> = h.highlight_line(text, &ss)?;

        let result = syntect::util::as_24_bit_terminal_escaped(&ranges[..], false);

        Terminal::write(result);

        Ok(())
    }

    /// Handle error
    fn handle_error(e: NanoError) -> NanoResult<()> {
        log::error!("{}", e);
        NanoEditor::exit()?;

        Ok(())
    }

    /// Exit terminal
    fn exit() -> NanoResult<()> {
        Terminal::reset()?;
        Terminal::clear()?;
        Terminal::flush()?;
        std::process::exit(0);
    }
}
