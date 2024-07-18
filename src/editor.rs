use std::fmt::format;
use std::thread::current;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::event::Event::Key;
use crate::bottom::BottomText;
use crate::error::print_error;
use crate::terminal::Terminal;
// 进行单词匹配查找时查询的方向
enum FindDirection {
    Forward,
    Backward,
}

// 默认位置
const DEFAULT_POSITION: (usize, usize) = (0, 0);

pub struct Editor {
    // 终端
    terminal: Terminal,
    // 光标位置
    cursor_position: (usize, usize),
    // 文本存储
    document: Document,
    // 底部状态栏
    bottom_text: BottomText,
    // 高亮设置
    highlight: Option<String>,
    // 偏移量
    offset: (usize, usize),
    // 是否退出
    quit: bool,
    // 记录退出次数进行强制退出
    force_quit: usize,
}

// 成员函数
impl Editor {
    // 程序运行循环函数，持续调用屏幕刷新和键盘输入处理函数
    // 当发生错误或是要退出时，退出循环
    pub fn run(&mut self) {
        loop {
            if let Err(e) = self.refresh_screen() {
                print_error(&e)
            }

            if self.quit {
                break;
            }

            if let Err(e) = self.handle_events() {
                print_error(&e);
            }
        }
    }

    // 处理键盘输入
    // 需要处理多种捕捉到的事件
    // Key: 键盘按键事件，这包括普通字符键、功能键、方向键等。
    // Mouse: 鼠标事件，包括鼠标点击、移动和滚轮滚动。
    // Resize: 终端窗口大小改变事件。
    // Paste: 剪切板输入事件。
    fn handle_events(&mut self) -> Result<(), std::io::Error> {
        // 是否进行额外的错误处理？
        let event = self.terminal.read()?;

        match event {
            Key(key_input) => {
                self.handle_key_event(key_input);
            },
            Event::Mouse(mouse_input) => {
                todo!("处理鼠标输入的事件")
            },
            Event::Resize(_, _) => {
                todo!("处理窗口大小改变的事件")
            },
            Event::Paste(paste_input) => {
                todo!("处理粘贴板输入的事件")
            },
            _ => {
                todo!("处理其他事件")
            }
        };
        Ok(())
    }

    // 处理键盘输入的事件
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match (key_event.modifiers, key_event.code) {
            // 处理控制键的事件
            (KeyModifiers::CONTROL, KeyCode::Char(c)) => {
                match c {
                    's' => {

                    },
                    'q' => {

                    }
                }
            },
            // 处理换行键的事件
            (_, KeyCode::Enter) => {
                todo!("处理换行键的事件");
            }
            // 处理退格键的事件
            (_, KeyCode::Backspace) => {
                todo!("处理删除键的事件");
            }
            // 处理删除键事件
            (_, KeyCode::Delete) => {

            }
            // 处理方向键
            (_, KeyCode::Up) | (_, KeyCode::Down) | (_, KeyCode::Left) | (_, KeyCode::Right) => {
                todo!("处理方向键的事件");
            },
            (_, KeyCode::Char(c)) => {
                self.document.insert(&self.cursor_position, c);
                self.move_cursor_position(KeyCode::Right);
            }
        }
    }
    // 处理鼠标输入的事件
    fn handle_mouse_event(&mut self, mouse_event: crossterm::event::Event::Mouse) {}
    // 处理窗口大小改变的事件
    fn handle_resize_event(&mut self, resize_event: crossterm::event::Event::Resize) {}
    //处理剪切板输入的事件
    fn handle_paste_event(&mut self, paste_event: crossterm::event::Event::Paste) {}

    // 移动光标位置
    fn move_cursor_position(&mut self, key_code: KeyCode) {
        let terminal_height = self.terminal.get_size().1;
        let (mut current_x, mut current_y) = self.cursor_position;
        todo!("获取文件的长度和行的长度");

        match key_code {
            KeyCode::Up => current_y = current_y.saturating_sub(1),
            KeyCode::Down => {

            },
            KeyCode::left => {

            },
            KeyCode::Right => {

            },
            _ => (),
        }
    }

    // 刷新屏幕显示输入
    pub fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor();
        // 将光标移动到默认的位置
        Terminal::move_cursor(DEFAULT_POSITION.0 as u16, DEFAULT_POSITION.1 as u16);
        // 如果字段选择退出，则退出
        if self.quit {
            Terminal::exit_terminal();
        } else {
            todo!("文本的高亮接口接入");
            todo!("文本显示接口、状态栏显示、消息显示");
            todo!("光标位置移动");
        }
        Terminal::show_cursor();
        Terminal::flush()
    }

    //保存文件
    fn save(&mut self) {
        todo!("document中没有文件名则提示输入文件名");
        todo!("保存文件")
    }

    // 查询指定的单词
    fn find(&mut self) {
        let old_cursor_position = self.cursor_position.clone();
        let mut direction = FindDirection::Forward;
        let find_text = self.status_message(
            "Find test(use ESC to cancel):",
            |editor, key, query_text| {
                let mut moved = false;
                match key.code {
                    KeyCode::Right | KeyCode::Down => {
                        direction = FindDirection::Forward;
                        editor.move_cursor_position(KeyCode::Right);
                        moved = true;
                    }
                    KeyCode::Left | KeyCode::Up => direction = FindDirection::Backward,
                    _ => direction = FindDirection::Forward,
                }

                if let Some(position) = editor.document.find(&query_text, &editor.cursor_position, direction) {
                    editor.cursor_position = position;
                    editor.scroll();
                } else if moved {
                    editor.move_cursor_position(KeyCode::Left);
                }
                editor.highlight = Some(query_text.to_string());
            },
        );

        if find_text.unwrap_or(None).is_none() {
            self.cursor_position = old_cursor_position;
            self.scroll();
        }
        self.highlight = None;
    }

    // 确保光标的可见性区域
    fn scroll(&mut self) {
        todo!("确保光标的可见性区域");
    }

    // 重新计算光标位置
    fn recalculate_cursor_position(&mut self, key_code: KeyCode) {
        todo!("重新计算光标位置");
        match key_code {
            KeyCode::Up => {
                todo!("向上移动光标");
            }
            KeyCode::Down => {
                todo!("向下移动光标");
            }
            KeyCode::Left => {
                todo!("向左移动光标");
            }
            KeyCode::Right => {
                todo!("向右移动光标");
            }
            _ => {
                todo!("其他操作");
            }
        }
    }

    // 用于在底部显示多种不同的提示信息
    fn status_message<Func> (&mut self, message: &str, callback: Func)
        -> Result<Option<String>, std::io::Error>
        where Func: FnMut(&mut Self, crossterm::event::KeyEvent, &String)
    {
        // result 存储用于底部控制栏输入的字符
        let mut result = String::new();
        loop{
            self.bottom_text = BottomText::from(format!("{}{}", message, result));
            self.refresh_screen()?;
            // 读取键盘输入
            let event = self.terminal.read()?;

            if let Key(key) = event {
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
        self.bottom_text = BottomText::from("".to_string());
        if result.is_empty() {
            return Ok(None);
        }
        Ok(Some(result))
    }

    // 将指定行文本绘制到终端
    // 如果打开的文件为空，则不用显示信息
    // 否则显示文本信息
    fn draw_lines(&mut self) {
        let terminal_height = self.terminal.get_size().1;
        for line_number in 0..terminal_height {
            Terminal::clear_line();
            todo!("根据文本的行号，显示文本信息")
        }
    }

    // 辅助显示函数
    fn show_help(self, line_text: String) {
        let width = self.terminal.get_size().0;
        let start = self.offset.0;
        let end = self.offset.0.saturating_add(width);
        todo!("显示行文本")
    }
}

// 方法
impl Editor {
    // 构造函数
    pub fn new() -> Self {
        // 获取需要打开的文件
        let args = std::env::args().collect::<Vec<_>>();
        let document = if let Some(file_name) = args.get(1) {
            todo!("用打开的文件创建一个text字段");
        } else {
            todo!("创建一个新的");
        };
        todo!("调用实现的document类");

        Self {
            terminal: Terminal::new().expect("Failed to create terminal"),
            cursor_position: (0, 0),
            document: document,
            bottom_text: BottomText::from("Hello World".to_string()),
            highlight: None,
            offset: DEFAULT_POSITION,
            quit: false,
            force_quit: 0,
        }
    }
}