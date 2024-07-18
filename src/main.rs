mod terminal;
mod error;
mod editor;
mod highlighting;
mod bottom;
mod doc;
mod row;
mod filetype;


pub use crossterm::event::Event;
#[derive(Default, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}


fn main() {
    let mut editor = editor::Editor::new();
    editor.run();
}