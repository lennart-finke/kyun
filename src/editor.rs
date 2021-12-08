use crate::Document;
use crate::Row;
use crate::Terminal;
use std::env;
use std::time::Duration;
use std::time::Instant;
use std::include_bytes;

use crossterm::{
    style::{Colors, Color},
    event::{Event, KeyCode, KeyModifiers, KeyEvent, read},
};

const STATUS_FG_COLOR: Color = Color::Rgb{r: 252, g: 196, b: 228};
const STATUS_BG_COLOR: Color = Color::Rgb{r: 153, g: 1, b: 87};
const QUIT_TIMES: u8 = 3;
const WELCOME_WIDTH : usize = 41;
#[derive(PartialEq, Copy, Clone)]
pub enum SearchDirection {
    Forward,
    Backward,
}

#[derive(Default, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

struct StatusMessage {
    text: String,
    time: Instant,
}
impl StatusMessage {
    fn from(message: String) -> Self {
        Self {
            time: Instant::now(),
            text: message,
        }
    }
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
    welcome_message: Document,
    status_message: StatusMessage,
    quit_times: u8,
    highlighted_word: Option<String>,
}

impl Editor {
    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }
    pub fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut initial_status =
            String::from("HEWP: Ctrl-F = find | Ctrl-S = save | Esc = qwit");

        let document = if let Some(file_name) = args.get(1) {
            let doc = Document::open(file_name);
            if let Ok(doc) = doc {
                doc
            } else {
                initial_status = format!("EWWOR!!! Could not open fiwe??! ＼＼(๑`^´๑)۶/怒／／ {}", file_name);
                Document::default()
            }
        } else {
            Document::default()
        };

        let welcome_bytes = include_bytes!("welcome.txt");
        let welcome_string = String::from_utf8(welcome_bytes.to_vec()).unwrap();
        let welcome = Document::from_string(welcome_string).unwrap();

        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            document,
            cursor_position: Position::default(),
            offset: Position::default(),
            status_message: StatusMessage::from(initial_status),
            welcome_message: welcome,
            quit_times: QUIT_TIMES,
            highlighted_word: None,
        }
    }

    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();

        Terminal::cursor_position(&Position::default());
        if self.should_quit {
            Terminal::quit();
        } else {
            self.document.highlight(
                &self.highlighted_word,
                Some(
                    self.offset
                        .y
                        .saturating_add(self.terminal.size().height as usize),
                ),
            );
            self.draw_rows();
            self.draw_status_bar();
            self.draw_message_bar();

            Terminal::cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            });
        }
        Terminal::cursor_show();
        Terminal::flush()
    }
    fn save(&mut self) {
        if self.document.file_name.is_none() {
            let new_name = self.prompt("Sawe as: ", |_, _, _| {}).unwrap_or(None);
            if new_name.is_none() {
                self.status_message = StatusMessage::from("Sawe aborted ; w ;.".to_string());
                return;
            }
            self.document.file_name = new_name;
        }

        if self.document.save().is_ok() {
            self.status_message = StatusMessage::from("Fiwe sawed successfuwwy. (- w -)ゞ".to_string());
        } else {
            self.status_message = StatusMessage::from("Error writing file! OWO".to_string());
        }
    }
    fn search(&mut self) {
        let old_position = self.cursor_position.clone();
        let mut direction = SearchDirection::Forward;
        let query = self
            .prompt(
                "Searching owo (ESC to cancel, Awwows to nawigate): ",
                |editor, key, query| {
                    let mut moved = false;
                    match key.code {
                        KeyCode::Right | KeyCode::Down => {
                            direction = SearchDirection::Forward;
                            editor.move_cursor(KeyCode::Right);
                            moved = true;
                        }
                        KeyCode::Left | KeyCode::Up => direction = SearchDirection::Backward,
                        _ => direction = SearchDirection::Forward,
                    }
                    if let Some(position) =
                        editor
                            .document
                            .find(&query, &editor.cursor_position, direction)
                    {
                        editor.cursor_position = position;
                        editor.scroll();
                    } else if moved {
                        editor.move_cursor(KeyCode::Left);
                    }
                    editor.highlighted_word = Some(query.to_string());
                },
            )
            .unwrap_or(None);

        if query.is_none() {
            self.cursor_position = old_position;
            self.scroll();
        }
        self.highlighted_word = None;
    }
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let event = Terminal::read(&mut self.terminal)?;

        if let Event::Key(pressed_key) = event {
            match (pressed_key.modifiers, pressed_key.code) {
                (KeyModifiers::CONTROL, KeyCode::Char('q')) | (_, KeyCode::Esc) => {
                    if self.quit_times > 0 && self.document.is_dirty() {
                        self.status_message = StatusMessage::from(format!(
                            "OwO! Fiwe has unsawed changes!!! Pwess Esc {} mowe times to qwit1!!!",
                            self.quit_times
                        ));
                        self.quit_times -= 1;
                        return Ok(());
                    }
                    self.should_quit = true
                }
                (KeyModifiers::CONTROL, KeyCode::Char('s')) => self.save(),
                (KeyModifiers::CONTROL, KeyCode::Char('f')) => self.search(),
                (KeyModifiers::CONTROL, KeyCode::Char('l')) => {
                    self.document.insert(&self.cursor_position, 'l');
                    self.move_cursor(KeyCode::Right);
                },
                (KeyModifiers::CONTROL, KeyCode::Char('r')) => {
                    self.document.insert(&self.cursor_position, 'r');
                    self.move_cursor(KeyCode::Right);
                },
                (KeyModifiers::CONTROL, KeyCode::Char('*')) => {
                    self.document.insert(&self.cursor_position, '*');
                    self.move_cursor(KeyCode::Right);
                },
                (_, KeyCode::Enter) => {
                    self.document.insert(&self.cursor_position, '\n');
                    self.move_cursor(KeyCode::Right);
                },
                (_, KeyCode::Char(mut c)) => {
                    match c {
                        'l' | 'r' => {
                            c = 'w';
                        },

                        'L' | 'R' => {
                            c = 'W';
                        },
                        '*' => {
                            self.document.insert(&self.cursor_position, '*');
                            self.document.insert(&self.cursor_position, ' ');
                            self.document.insert(&self.cursor_position, 's');
                            self.document.insert(&self.cursor_position, 'e');
                            self.document.insert(&self.cursor_position, 'c');
                            self.document.insert(&self.cursor_position, 'i');
                            self.document.insert(&self.cursor_position, 't');
                            self.document.insert(&self.cursor_position, 'o');
                            self.document.insert(&self.cursor_position, 'n');
                            self.document.insert(&self.cursor_position, '*');

                            self.move_right(10);
                            return Ok(());
                        },

                        'U' => {
                            self.document.insert(&self.cursor_position, 'U');
                            self.document.insert(&self.cursor_position, 'w');
                            self.document.insert(&self.cursor_position, 'U');

                            self.move_right(4);
                            return Ok(());
                        },

                        'O' => {
                            self.document.insert(&self.cursor_position, 'O');
                            self.document.insert(&self.cursor_position, 'w');
                            self.document.insert(&self.cursor_position, 'O');

                            self.move_right(4);
                            return Ok(());
                        }
                        _ => {}
                    }

                    self.document.insert(&self.cursor_position, c);
                    self.move_cursor(KeyCode::Right);
                }
                (_, KeyCode::Delete) => self.document.delete(&self.cursor_position),
                (_, KeyCode::Backspace) => {
                    if self.cursor_position.x > 0 || self.cursor_position.y > 0 {
                        self.move_cursor(KeyCode::Left.into());
                        self.document.delete(&self.cursor_position);
                    }
                },
                (_, KeyCode::Up)
                | (_, KeyCode::Down)
                | (_, KeyCode::Left)
                | (_, KeyCode::Right)
                | (_, KeyCode::PageUp)
                | (_, KeyCode::PageDown)
                | (_, KeyCode::End)
                | (_, KeyCode::Home) => self.move_cursor(pressed_key.code),
                _ => (),
            }
            self.scroll();
            if self.quit_times < QUIT_TIMES {
                self.quit_times = QUIT_TIMES;
                self.status_message = StatusMessage::from(String::new());
            }
        }

        else if let Event::Resize(width, height) = event {
            self.terminal.size.width = width;
            if env::consts::OS == "windows" {
                self.terminal.size.height = height - 1;
            }
            else {
                self.terminal.size.height = height - 2;
            }
        }


        Ok(())
    }
    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let mut offset = &mut self.offset;
        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }
        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }
    fn move_cursor(&mut self, key: KeyCode) {
        let terminal_height = self.terminal.size().height as usize;
        let Position { mut y, mut x } = self.cursor_position;
        let height = self.document.len();
        let mut width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        match key {
            KeyCode::Up => y = y.saturating_sub(1),
            KeyCode::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            KeyCode::Left => {
                if x > 0 {
                    x -= 1;
                } else if y > 0 {
                    y -= 1;
                    if let Some(row) = self.document.row(y) {
                        x = row.len();
                    } else {
                        x = 0;
                    }
                }
            }
            KeyCode::Right => {
                if x < width {
                    x += 1;
                } else if y < height {
                    y += 1;
                    x = 0;
                }
            }
            KeyCode::PageUp => {
                y = if y > terminal_height {
                    y.saturating_sub(terminal_height)
                } else {
                    0
                }
            }
            KeyCode::PageDown => {
                y = if y.saturating_add(terminal_height) < height {
                    y.saturating_add(terminal_height)
                } else {
                    height
                }
            }
            KeyCode::Home => x = 0,
            KeyCode::End => x = width,
            _ => (),
        }
        width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        if x > width {
            x = width;
        }

        self.cursor_position = Position { x, y }
    }

    fn move_right(&mut self, i: u8) {
        for _ in 1..i {
            self.move_cursor(KeyCode::Right);
        }
    }

    fn draw_centered(&self, r: &Row) {
        let mut width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x.saturating_add(width);
        let mut row = r.render(start, end);

        let padding = width.saturating_sub(WELCOME_WIDTH) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        row = format!("{}{}", spaces, row);

        if row.len() > width  {
            while width > 0 {
                if row.is_char_boundary(width) {
                    row.truncate(width);

                    break;
                }
                width -= 1;
            }
        }

        println!("{}\r", row);
    }
    pub fn draw_row(&self, row: &Row) {
        let width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x.saturating_add(width);
        let row = row.render(start, end);
        println!("{}\r", row)
    }
    fn draw_rows(&self) {
        let height = self.terminal.size().height;

        for terminal_row in 0..height {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(self.offset.y.saturating_add(terminal_row as usize)) {
                self.draw_row(row);
            }

            else if self.document.is_empty() {
                if let Some(row) = self.welcome_message
                    .row(self.offset.y.saturating_add(terminal_row as usize))
                {
                    self.draw_centered(row);
                }
            } else {
                println!("\r");
            }
        }
    }
    fn draw_status_bar(&self) {
        let mut status;
        let width = self.terminal.size().width as usize;
        let modified_indicator = if self.document.is_dirty() {
            " (modified)"
        } else {
            ""
        };

        let mut file_name = "[uwunamed]".to_string();
        if let Some(name) = &self.document.file_name {
            file_name = name.clone();
            file_name.truncate(20);
        }
        status = format!(
            "{} - {} lines{}",
            file_name,
            self.document.len(),
            modified_indicator
        );

        let line_indicator = format!(
            "{} | {}/{}",
            self.document.file_type(),
            self.cursor_position.y.saturating_add(1),
            self.document.len()
        );
        let len = status.len() + line_indicator.len();
        status.push_str(&" ".repeat(width.saturating_sub(len)));
        status = format!("{}{}", status, line_indicator);
        status.truncate(width);

        Terminal::set_colors(Colors::new(STATUS_BG_COLOR, STATUS_FG_COLOR));

        println!("{}\r", status);
        Terminal::reset_colors();
    }
    fn draw_message_bar(&self) {
        Terminal::clear_current_line();
        let message = &self.status_message;
        if Instant::now() - message.time < Duration::new(5, 0) {
            let mut text = message.text.clone();
            text.truncate(self.terminal.size().width as usize);
            print!("{}", text);
        }
    }
    fn prompt<C>(&mut self, prompt: &str, mut callback: C) -> Result<Option<String>, std::io::Error>
    where
        C: FnMut(&mut Self, KeyEvent, &String),
    {
        let mut result = String::new();
        loop {
            self.status_message = StatusMessage::from(format!("{}{}", prompt, result));
            self.refresh_screen()?;
            let event = read().unwrap();

            if let Event::Key(key) = event {
                match key.code {
                    KeyCode::Backspace => result.truncate(result.len().saturating_sub(1)),
                    KeyCode::Enter => break,
                    KeyCode::Char(c) => {
                        if !c.is_control() {
                            result.push(c);
                        }
                    }
                    KeyCode::Esc => {
                        result.truncate(0);
                        break;
                    }
                    _ => (),
                }
                callback(self, key, &result);
            }

        }
        self.status_message = StatusMessage::from(String::new());
        if result.is_empty() {
            return Ok(None);
        }
        Ok(Some(result))
    }
}

fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
