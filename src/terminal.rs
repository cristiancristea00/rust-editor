use std::io;
use std::io::{Error, Write};

use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

use crate::editor::Position;

pub struct Size {
    width: u16,
    height: u16,
}

impl Size {
    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }
}

pub struct Terminal {
    size: Size,
    raw_terminal: RawTerminal<io::Stdout>,
}

impl Default for Terminal {
    fn default() -> Self {
        let (width, height) = termion::terminal_size().expect("Failed to get terminal size");

        let height = height.saturating_sub(2);

        Self {
            size: Size { width, height },
            raw_terminal: io::stdout().into_raw_mode().expect("Failed to get stdout"),
        }
    }
}

impl Terminal {
    #[must_use]
    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    pub fn set_bg_colour(color: color::Rgb) {
        print!("{}", color::Bg(color));
    }

    pub fn reset_bg_colour() {
        print!("{}", color::Bg(color::Reset));
    }

    pub fn set_fg_colour(color: color::Rgb) {
        print!("{}", color::Fg(color));
    }

    pub fn reset_fg_colour() {
        print!("{}", color::Fg(color::Reset));
    }

    pub fn cursor_position(position: &Position) {
        let Position { mut x, mut y } = position;
        x = x.saturating_add(1);
        y = y.saturating_add(1);
        let x = u16::try_from(x);
        let y = u16::try_from(y);

        match (x, y) {
            (Ok(x), Ok(y)) => print!("{}", termion::cursor::Goto(x, y)),
            _ => panic!("Failed to convert cursor position to u16"),
        }
    }

    pub fn flush() -> Result<(), Error> {
        io::stdout().flush()
    }

    pub fn read_key() -> Result<Key, Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }
}
