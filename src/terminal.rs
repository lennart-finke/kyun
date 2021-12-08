use crate::Position;
use std::io::{stdout, Write};

use crossterm::{
    style::{Color, SetForegroundColor, Colors, SetColors, ResetColor},
    ExecutableCommand,
    QueueableCommand,
    event::{read, Event},
    cursor,
    terminal,
};

pub struct Size {
    pub width: u16,
    pub height: u16,
}
pub struct Terminal {
    pub size: Size,
    color: usize,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = terminal::size().unwrap();
        terminal::enable_raw_mode().ok();

        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1.saturating_sub(2),
            },
            color: 190
        })
    }
    pub fn size(&self) -> &Size {
        &self.size
    }
    pub fn quit() {
        Terminal::reset_colors();
        stdout().execute(terminal::Clear(terminal::ClearType::All)).ok();
        crossterm::terminal::disable_raw_mode().ok();

        println!("\n\t            ^   ^    \n\tBye bye ヾ(｡>﹏<｡)ﾉ\r\n");


    }
    pub fn clear_screen() {
        stdout().execute(terminal::Clear(terminal::ClearType::All)).ok();
    }

    pub fn cycle_colors(&mut self) {
        self.color += 1;

        if self.color > 230 {
            self.color = 190;
        }

        stdout().execute(SetForegroundColor(Color::AnsiValue(self.color as u8))).ok();
    }

    pub fn cursor_position(position: &Position) {
        let Position { mut x, mut y } = position;
        x = x.saturating_add(1);
        y = y.saturating_add(1);
        let x = x as u16;
        let y = y as u16;

        stdout().queue(cursor::MoveTo(x - 1, y - 1)).ok();
    }
    pub fn flush() -> Result<(), std::io::Error> {
        stdout().flush()
    }
    pub fn read(&mut self) -> Result<Event, std::io::Error> {
        loop {
            let event = read();

            if let Ok(Event::Key(_)) = event {
                self.cycle_colors();
            }

            return event
        }
    }


    pub fn cursor_hide() {
        stdout().execute(cursor::DisableBlinking).ok();
    }
    pub fn cursor_show() {
        stdout().execute(cursor::EnableBlinking).ok();
    }
    pub fn clear_current_line() {
        stdout().execute(terminal::Clear(terminal::ClearType::CurrentLine)).ok();
    }
    pub fn set_colors(colors: Colors) {
        stdout().execute(SetColors(colors)).ok();
    }
    pub fn reset_colors() {
        stdout().execute(ResetColor).ok();
    }
}
