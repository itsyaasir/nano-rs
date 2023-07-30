use crossterm::event::{self, read, Event, KeyCode, KeyEvent, MouseEvent};
use crossterm::style::{self, Color, ContentStyle, PrintStyledContent, StyledContent};
use crossterm::style::{style, Stylize};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{cursor, terminal};
use crossterm::{execute, queue};
use std::error::Error;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::Duration;
use std::{env, fs, io};
use syntect::easy::HighlightFile;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<String>>();
    let file_name = PathBuf::from_str(&args[1])?;
    let mut stdout = io::stdout();
    run(&mut stdout, &file_name)?;
    Ok(())
}

fn run<W>(w: &mut W, file_name: &PathBuf) -> Result<(), Box<dyn Error>>
where
    W: std::io::Write,
{
    execute!(w, EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut should_print_file = true;
    loop {
        if should_print_file {
            let (width, _height) = terminal::size()?;
            queue!(w, terminal::Clear(terminal::ClearType::All))?;
            // Print the file name at the center of the first line
            queue!(
                w,
                cursor::MoveTo(
                    (width / 2) - (file_name.to_str().unwrap().len() as u16 / 2),
                    0
                )
            )?;
            queue!(
                w,
                PrintStyledContent(
                    style(file_name.to_str().unwrap())
                        .with(Color::Black)
                        .on_white()
                )
            )?;
            print_file_contents(w, file_name)?;
            should_print_file = false;
        }

        if event::poll(Duration::from_millis(500))? {
            match read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                }) => break,
                Event::Mouse(MouseEvent {
                    kind, column, row, ..
                }) => match kind {
                    event::MouseEventKind::ScrollDown | event::MouseEventKind::ScrollUp => {}
                    _ => {
                        queue!(w, cursor::MoveTo(column, row))?;
                        should_print_file = true;
                    }
                },
                _ => {}
            }
        }
        w.flush()?;
    }

    disable_raw_mode()?;
    execute!(w, LeaveAlternateScreen)?;
    execute!(w, terminal::Clear(terminal::ClearType::All))?;
    Ok(())
}

fn print_file_contents<W: std::io::Write>(w: &mut W, file_name: &PathBuf) -> io::Result<()> {
    let file_contents = fs::read_to_string(file_name)?;
    let mut line_number = 1;
    for line in file_contents.lines() {
        queue!(w, cursor::MoveTo(0, line_number))?;
        let ss = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        let mut highlighter =
            HighlightFile::new(file_name, &ss, ts.themes.get("base16-ocean.dark").unwrap())?;
        let ranges: Vec<(syntect::highlighting::Style, &str)> = highlighter
            .highlight_lines
            .highlight_line(line, &ss)
            .unwrap();
        for (text_style, text) in ranges {
            let fg = text_style.foreground;
            let bg = text_style.background;
            let fg = Color::Rgb {
                r: fg.r,
                g: fg.g,
                b: fg.b,
            };
            let bg = Color::Rgb {
                r: bg.r,
                g: bg.g,
                b: bg.b,
            };
            let styled_content = style(text).with(fg).on(bg);
            queue!(w, PrintStyledContent(styled_content))?;
        }
        line_number += 1;
    }

    Ok(())
}
