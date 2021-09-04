mod screen;
mod editor;
mod world;

pub use world::*;
pub use editor::*;
pub use screen::*;

#[derive(PartialEq, Eq)]
pub enum SubRendererResult {
    None,
    NeedsRepaint,
    Quit
}