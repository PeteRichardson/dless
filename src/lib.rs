use clap::Parser;
use crossterm::{
    event::{self},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    // prelude::{Color, Style},
    prelude::{CrosstermBackend, Terminal},
    widgets::{Block, Borders},
};
use std::fs::File;

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::process::ExitCode;
use tui_textarea::{CursorMove, Input, Key, Scrolling, TextArea};

#[derive(Parser, Debug, Clone)]
#[command(version, about)]
pub struct DlessConfig {
    /// log file to view
    #[arg(default_value = "testdata/dlog0.log")]
    pub file: String,
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn dless(config: &DlessConfig) -> std::result::Result<ExitCode, Box<dyn std::error::Error>> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let textlog: Vec<String> = lines_from_file(&config.file);

    let mut textarea = TextArea::from(textlog);
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            //.style(Style::default().fg(Color::Black).bg(Color::White))
            .title(config.file.clone()),
    );

    loop {
        terminal.draw(|f| {
            f.render_widget(textarea.widget(), f.size());
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            let input = event::read()?.into();
            match input {
                Input {
                    key: Key::Char('q'),
                    ..
                }
                | Input { key: Key::Esc, .. } => break,
                Input {
                    key: Key::Char('h'),
                    ..
                } => textarea.move_cursor(CursorMove::Back),
                Input {
                    key: Key::Char('j'),
                    ..
                } => textarea.move_cursor(CursorMove::Down),
                Input {
                    key: Key::Char('k'),
                    ..
                } => textarea.move_cursor(CursorMove::Up),
                Input {
                    key: Key::Char('l'),
                    ..
                } => textarea.move_cursor(CursorMove::Forward),
                Input {
                    key: Key::Char('w'),
                    ..
                } => textarea.move_cursor(CursorMove::WordForward),
                Input {
                    key: Key::Char('b'),
                    ctrl: false,
                    ..
                } => textarea.move_cursor(CursorMove::WordBack),
                Input {
                    key: Key::Char('^'),
                    ..
                } => textarea.move_cursor(CursorMove::Head),
                Input {
                    key: Key::Char('$'),
                    ..
                } => textarea.move_cursor(CursorMove::End),
                Input {
                    key: Key::Char('g'),
                    ctrl: false,
                    ..
                }
                | Input { key: Key::Home, .. } => textarea.move_cursor(CursorMove::Top),
                Input {
                    key: Key::Char('G'),
                    ctrl: false,
                    ..
                }
                | Input { key: Key::End, .. } => textarea.move_cursor(CursorMove::Bottom),
                Input {
                    key: Key::Char('e'),
                    ctrl: true,
                    ..
                } => textarea.scroll((1, 0)),
                Input {
                    key: Key::Char('y'),
                    ctrl: true,
                    ..
                } => textarea.scroll((-1, 0)),
                Input {
                    key: Key::Char('d'),
                    ctrl: true,
                    ..
                } => textarea.scroll(Scrolling::HalfPageDown),
                Input {
                    key: Key::Char('u'),
                    ctrl: true,
                    ..
                } => textarea.scroll(Scrolling::HalfPageUp),
                Input {
                    key: Key::Char('f'),
                    ctrl: true,
                    ..
                }
                | Input {
                    key: Key::PageDown, ..
                } => textarea.scroll(Scrolling::PageDown),
                Input {
                    key: Key::Char('b'),
                    ctrl: true,
                    ..
                }
                | Input {
                    key: Key::PageUp, ..
                } => textarea.scroll(Scrolling::PageUp),

                _ => (),
            }
        }
    }

    disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(ExitCode::SUCCESS)
}
