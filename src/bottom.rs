use std::time::Instant;

// 底部文本行显示的提示信息
pub struct BottomText {
    // 显示的内容设置
    text: String,
    // 上次修改的时间
    time: Instant,
}

impl From<String> for BottomText {
    fn from(text: String) -> Self {
        Self {
            text,
            time: Instant::now(),
        }
    }
}