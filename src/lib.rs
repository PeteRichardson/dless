use clap::Parser;
use crossterm::{
    event::{self, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    widgets::Paragraph,
};
use std::fs::read_to_string;

use std::io::stdout;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser, Debug, Clone)]
#[command(version, about)]
pub struct DlessConfig {
    /// log file to view
    #[arg(default_value = "testdata/dlog0.log")]
    pub file: PathBuf,
}

pub fn dless(config: &DlessConfig) -> std::result::Result<ExitCode, Box<dyn std::error::Error>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let s: String = read_to_string(&config.file)?;
    let mut x_offset: u16 = 0;
    let mut y_offset: u16 = 0;

    loop {
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Up | KeyCode::Char('j') => {
                        y_offset += 1;
                    }
                    KeyCode::Down | KeyCode::Char('k') => {
                        if y_offset > 0 {
                            y_offset -= 1;
                        }
                    }
                    KeyCode::Right | KeyCode::Char('l') => {
                        x_offset += 1;
                    }
                    KeyCode::Left | KeyCode::Char('h') => {
                        if x_offset > 0 {
                            x_offset -= 1;
                        }
                    }
                    _ => {}
                }
            }
        }

        terminal.draw(|frame| {
            let area = frame.size();
            let p = Paragraph::new(s.clone()).scroll((y_offset, x_offset));
            frame.render_widget(p, area);
        })?;
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(ExitCode::SUCCESS)
}
