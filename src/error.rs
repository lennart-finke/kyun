use crate::terminal::Terminal;

#[macro_export]
macro_rules! handle_error {
    ($expr:expr) => {
        match $expr {
            Ok(_) => {},
            Err(e) => {
                // Todo: log error
            }
        }
    };
}

// 打印错误信息到终端上
pub fn print_error(e: &std::io::Error) {
    Terminal::clear_terminal();
    print!("Error: {}", e);
}
