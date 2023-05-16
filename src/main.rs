use ansi_term::Colour::{Fixed, White};
use clap::Parser;
use console::Term;
use crossterm::event::{read, Event, KeyCode};
use crossterm::{cursor, terminal};
use std::error::Error;
use std::io::Write;
use std::io::{BufRead, ErrorKind, StdoutLock};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{env, fs, io};
use syntect::easy::HighlightFile;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::as_24_bit_terminal_escaped;

const PANEL_WIDTH: usize = 7;
const GRID_COLOR: u8 = 238;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct NanoArgs {
    /// The file to open
    #[clap(short, long)]
    pub file: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = NanoArgs::parse();
    let file_name = PathBuf::from_str(&args.file)?;

    let home_dir = env::home_dir().ok_or(io::Error::new(
        ErrorKind::Other,
        "Could not get home directory",
    ))?;

    let theme_dir = home_dir.join(".config").join("nano").join("themes");
    let theme_set = ThemeSet::load_from_folder(theme_dir)
        .map_err(|_| io::Error::new(ErrorKind::Other, "Could not load themes"))?;
    let theme = &theme_set.themes["Monokai"];

    let syntax_set = SyntaxSet::load_defaults_nonewlines();

    print_file(theme, &syntax_set, file_name)?;

    Ok(())
}

fn print_horizontal_line(
    handle: &mut StdoutLock,
    grid_char: char,
    term_width: usize,
) -> io::Result<()> {
    let bar = "─".repeat(term_width - (PANEL_WIDTH + 1));
    let line = format!("{}{}{}", "─".repeat(PANEL_WIDTH), grid_char, bar);

    writeln!(handle, "{}", Fixed(GRID_COLOR).paint(line))?;

    Ok(())
}

fn print_file<P: AsRef<Path>>(
    theme: &Theme,
    syntax_set: &SyntaxSet,
    filename: P,
) -> io::Result<()> {
    let mut highlighter = HighlightFile::new(filename.as_ref(), syntax_set, theme)?;

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let term = Term::stdout();
    let (_, term_width) = term.size();
    let term_width = term_width as usize;

    print_horizontal_line(&mut handle, '┬', term_width)?;

    writeln!(
        handle,
        "{}{} {}",
        " ".repeat(PANEL_WIDTH),
        Fixed(GRID_COLOR).paint("│"),
        White.bold().paint(filename.as_ref().to_string_lossy())
    )?;

    print_horizontal_line(&mut handle, '┼', term_width)?;

    for (idx, maybe_line) in highlighter.reader.lines().enumerate() {
        let line_nr = idx + 1;
        let line = maybe_line.unwrap_or("<INVALID UTF-8>".into());
        let regions = highlighter
            .highlight_lines
            .highlight_line(&line, syntax_set)
            .unwrap();

        writeln!(
            handle,
            "{} {}  {}",
            Fixed(244).paint(format!("{:4}", line_nr)),
            Fixed(GRID_COLOR).paint("│"),
            as_24_bit_terminal_escaped(&regions, false)
        )?;
    }

    print_horizontal_line(&mut handle, '┴', term_width)?;

    Ok(())
}
