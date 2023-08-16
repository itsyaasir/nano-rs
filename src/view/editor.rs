use std::env;
use std::path::PathBuf;
use std::str::FromStr;

use crossterm::event::KeyCode;
use crossterm::style::{Color, Stylize};
use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;

use crate::config::configuration::NanoConfiguration;
use crate::content::Content;
use crate::error::{NanoError, NanoResult};
use crate::file::FileDocument;
use crate::view::terminal::TerminalView;
use crate::view::Position;

pub const NANO_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The Nano editor
///
/// This is the main editor struct. It contains the terminal view, the file
/// being edited, and the cursor position.
/// It also contains the offset of the view, which is used to scroll the view.
///
#[derive(Debug, Clone)]
pub struct NanoEditor {
    /// The terminal view
    terminal_view: TerminalView,

    /// The file being edited/viewed
    file: FileDocument,
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
    pub fn new() -> NanoResult<Self> {
        let args = env::args().collect::<Vec<String>>();
        let file_name =
            PathBuf::from_str(&args[1]).map_err(|e| NanoError::FileError(e.to_string()))?;
        let file = FileDocument::from_file(file_name)?;
        let terminal_view = TerminalView::new()?;

        let config = NanoConfiguration::parse().expect("failed to parse");
        Ok(Self {
            terminal_view,
            file,
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
        TerminalView::set_title(&format!(
            "Nano - {}",
            self.file
                .file_name
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
            self.file
                .file_name
                .as_ref()
                .unwrap_or(&String::from("Untitled")),
        )
        .with(Color::Black)
        .on(Color::White)
        .to_string();

        // Get the terminal width and the text length
        let terminal_width = self.terminal_view.width;
        let text_length = status_bar_message.len();

        // Calculate the number of spaces to add on each side of the text
        let num_spaces = (terminal_width as usize - text_length) / 2;
        let centered_text = format!(
            "{:>width$}",
            status_bar_message,
            width = text_length + num_spaces
        );

        TerminalView::write(centered_text);

        Ok(())
    }

    /// Process the key event captured from the terminal
    pub fn process_key(&mut self) -> NanoResult<()> {
        let event = self.terminal_view.read_key()?;

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
        let Position { mut x, mut y } = self.terminal_view.cursor;
        let document_height = self.file.len() as u16;
        let document_width = self.file.row(y as usize).map_or(0, |content| content.len()) as u16;

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

        self.terminal_view.set_cursor_position((x, y).into())
    }

    /// Render the editor
    /// This will render the editor, including the file, cursor, and status bar.
    ///
    fn render(&mut self) -> NanoResult<()> {
        TerminalView::hide_cursor()?;

        self.draw_status_bar()?;
        self.render_contents()?;

        self.terminal_view.set_cursor_position(Position {
            x: self
                .terminal_view
                .cursor
                .x
                .saturating_sub(self.terminal_view.offset.x),
            y: self
                .terminal_view
                .cursor
                .y
                .saturating_sub(self.terminal_view.offset.y),
        });

        TerminalView::show_cursor()?;
        TerminalView::flush()?;
        Ok(())
    }

    fn render_contents(& mut self) -> NanoResult<()> {
        let height = self.terminal_view.height;

        for terminal_row in 0..height {
            TerminalView::clear_current_line()?;

            if let Some(content) = self
                .file
                .row(terminal_row as usize + self.terminal_view.offset.y as usize)
            {
                self.render_content(content, terminal_row)?
            } else {
                TerminalView::write("~\r");
            }
        }

        if self.config.turn_on_line_numbers().expect("failed to turn on line numbers"){
            self.terminal_view.set_cursor_position(Position {
                x: self.terminal_view.size().0 - 10,
                y: self.terminal_view.cursor.y.saturating_sub(self.terminal_view.offset.y),
            });

            for i in 0..height{
                TerminalView::clear_current_line()?;
                TerminalView::write(format!("{:>3}\r", i + self.terminal_view.offset.y));
            }
        }

        Ok(())
    }

    fn render_content(&self, content: &Content, _line_number: u16) -> NanoResult<()> {
        let width = self.terminal_view.width as usize;
        let start = self.terminal_view.offset.x as usize;
        let end = self.terminal_view.offset.x as usize + width;
        let text = &content.display_range(start, end);

        let ss = SyntaxSet::load_defaults_newlines();
        let syntax =
            ss.find_syntax_by_extension(self.file.file_type())
                .ok_or(NanoError::Generic(format!(
                    "Syntax not found for {}",
                    self.file.file_type()
                )))?;

        let theme = &self.config.load_themes().expect("failed to load theme");

        let mut h = HighlightLines::new(syntax, theme);

        let ranges: Vec<(syntect::highlighting::Style, &str)> = h.highlight_line(text, &ss)?;

        let result = syntect::util::as_24_bit_terminal_escaped(&ranges[..], false);

        TerminalView::write(result);

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
        TerminalView::reset()?;
        TerminalView::clear()?;
        TerminalView::flush()?;
        std::process::exit(0);
    }
}
