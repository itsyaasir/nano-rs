mod buffer;
mod config;
pub mod content;
mod error;
mod view;
use std::fs::File;
use std::io::Write;
pub mod editor;
pub mod terminal;
use chrono::Local;
use env_logger::Target;
use error::NanoResult;
use log::LevelFilter;

use crate::config::configuration::NanoConfiguration;
use crate::editor::NanoEditor;

fn main() -> NanoResult<()> {
    init_logging();

    log::info!("Starting Nano");
    let configuration = NanoConfiguration::load();
    NanoEditor::new(configuration)?.run()?;
    Ok(())
}

/// Initialize logging
/// This will initialize the logger with the following settings:
/// - The log level is set to `Debug`
/// - The log format is set to `"{h({d(%Y-%m-%d %H:%M:%S)} {l})} {m}{n}"`
/// - The log output is set to `nano.log`
pub fn init_logging() {
    let target = Box::new(File::create("nano.log").expect("Can't create file"));

    env_logger::Builder::new()
        .target(Target::Pipe(target))
        .filter(None, LevelFilter::Debug)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {} {}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();
}
