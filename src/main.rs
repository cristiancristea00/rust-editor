pub use document::Document;
use editor::Editor;
pub use row::Row;
pub use terminal::Terminal;

mod document;
mod editor;
mod row;
mod terminal;

fn main() {
    Editor::default().run();
}
