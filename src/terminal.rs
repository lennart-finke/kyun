use std::io::{stdout, Write};
use crossterm::{execute, style::{
    Color,
    Colors,
}, terminal, event::{
    Event,
}, queue};
use crate::handle_error;

pub struct Terminal {
    width: usize,
    height: usize,
    color: u8,
}

impl Terminal {
    // 读取文本
    pub fn read(&mut self) -> Result<Event, std::io::Error> {
        loop {
            let event = crossterm::event::read();
            // 如果是键盘输入事件，改变文本的颜色
            if let Event::Key(_) = event {
                self.set_color();
            }
            return event;
        }
    }

    // 获取终端大小
    pub fn get_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    // 设置颜色
    pub fn set_color(&mut self) {
        self.color += 1;
        if self.color > 231 {
            self.color = 20;
        }
        Self::set_foreground_color(Color::AnsiValue(self.color))
    }
}


impl Terminal {
    //构造函数
    pub fn new() -> Result<Self, std::io::Error> {
        let (current_width, current_height) = terminal::size().unwrap();
        handle_error!(terminal::enable_raw_mode());

        Ok(Self{
            width: current_width as usize,
            height: current_height as usize,
            color: 20,
        })
    }

    // 设置终端全部颜色
    pub fn terminal_color(colors: Colors) {
        handle_error!(execute!(stdout(), crossterm::style::SetColors(colors)));
    }

    //设置终端前景色：字体颜色
    pub fn set_foreground_color(color: Color) {
        handle_error!(execute!(stdout(), crossterm::style::SetForegroundColor(color)));
    }

    // 刷新终端缓冲区
    pub fn flush() ->Result<(), std::io::Error> {
        stdout().flush()
    }

    // 隐藏终端光标
    pub fn hide_cursor() {
        handle_error!(execute!(stdout(), crossterm::cursor::Hide));
    }

    // 显示终端光标
    pub fn show_cursor() {
        handle_error!(execute!(stdout(), crossterm::cursor::Show));
    }

    // 清除终端行
    pub fn clear_line() {
        handle_error!(execute!(stdout(),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine)));
    }

    // 清除终端全部
    pub fn clear_terminal() {
        handle_error!(
            execute!(
                stdout(),
                crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
            )
        );
    }

    //重置终端颜色
    pub fn reset_color() {
       handle_error!(execute!(stdout(), crossterm::style::ResetColor));
    }

    // 重置终端的模式
    pub fn reset_terminal_mode() {
        handle_error!(execute!(stdout(), crossterm::terminal::LeaveAlternateScreen));
    }

    // 退出终端
    pub fn exit_terminal() {
        Self::reset_terminal_mode();
        Self::reset_color();
        Self::clear_terminal();
    }

    // 移动光标位置
    pub fn move_cursor(x: u16, y: u16) {
        handle_error!(execute!(stdout(), crossterm::cursor::MoveTo(x, y)))
    }
}