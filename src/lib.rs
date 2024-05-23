use clap::Parser;
use crossterm::event::{self};
use ratatui::{
    prelude::{Backend, Color, Style, Terminal},
    widgets::{Block, Borders},
};
use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};
use tui_textarea::{CursorMove, Input, Key, Scrolling, TextArea};

#[derive(Parser, Debug, Clone)]
#[command(version, about)]
pub struct DlessConfig {
    /// log file to view
    #[arg(default_value = "testdata/dlog0.log")]
    pub filename: String,
}

/// App holds the state of the application
pub struct App {
    filename: String,   // name of the log file to view
    lines: Vec<String>, // lines of the log file
}

impl App {
    pub fn new(config: &DlessConfig) -> Self {
        let file = File::open(config.filename.clone()).expect("no such file");
        let buf = BufReader::new(file);
        let textloglines = buf
            .lines()
            .map(|l| l.expect("Could not parse line"))
            .collect();

        Self {
            filename: config.filename.to_owned(),
            lines: textloglines,
        }
    }
}

fn ui<'a>(app: App) -> TextArea<'a> {
    let mut textarea = TextArea::from(app.lines.clone());
    textarea.set_line_number_style(Style::default().fg(Color::DarkGray));
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title(app.filename.clone()),
    );
    textarea
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: App) -> Result<()> {
    let mut textarea = ui(app).to_owned();

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
    Ok(())
}
