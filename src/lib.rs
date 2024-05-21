use clap::Parser;
use crossterm::{
    event::{self, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    widgets::{Block, Borders},
};
use std::fs::File;

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::process::ExitCode;
use tui_textarea::{CursorMove, Scrolling, TextArea};

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
            .title(config.file.clone()),
    );

    loop {
        terminal.draw(|f| {
            f.render_widget(textarea.widget(), f.size());
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up | KeyCode::Char('j') => textarea.move_cursor(CursorMove::Down),
                    KeyCode::Down | KeyCode::Char('k') => textarea.move_cursor(CursorMove::Up),
                    KeyCode::Right | KeyCode::Char('l') => {
                        textarea.move_cursor(CursorMove::Forward)
                    }
                    KeyCode::Left | KeyCode::Char('h') => textarea.move_cursor(CursorMove::Back),
                    KeyCode::Char('w') => textarea.move_cursor(CursorMove::WordForward),
                    //KeyCode::Char('b') => textarea.move_cursor(CursorMove::WordBack),
                    KeyCode::Char('^') => textarea.move_cursor(CursorMove::Head),
                    KeyCode::Char('$') => textarea.move_cursor(CursorMove::End),
                    KeyCode::Char('a') => textarea.move_cursor(CursorMove::Forward),
                    KeyCode::Char('A') => textarea.move_cursor(CursorMove::End),

                    // all the following should have ctrl = true
                    KeyCode::Char('e') => textarea.scroll((1, 0)),
                    KeyCode::Char('y') => textarea.scroll((-1, 0)),
                    KeyCode::Char('d') => textarea.scroll(Scrolling::HalfPageDown),
                    KeyCode::Char('u') => textarea.scroll(Scrolling::HalfPageUp),
                    KeyCode::Char('f') => textarea.scroll(Scrolling::PageDown),
                    KeyCode::Char('b') => textarea.scroll(Scrolling::PageUp),
                    _ => (),
                }
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
