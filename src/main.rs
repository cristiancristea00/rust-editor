use editor::Editor;
pub use terminal::Terminal;

mod editor;
mod terminal;

fn main() {
    Editor::default().run();
}
