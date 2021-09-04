mod editor;
mod system;

use crate::math;
use crate::input;

pub use editor::*;
pub use system::*;

#[derive(Debug)]
pub enum SystemEvent {
    Unused,
    CloseRequested,
    KeyboardAction { state: input::InputState, button: input::KeyboardButton },
    Resized(math::Size2d),
    MouseButtonAction { state: input::InputState, button: input::MouseButton },
    MouseWheelAction { delta: math::Vector2 },
    PointerMoved { position: math::Vector2 },
    EditorChange(editor::EditorEvent),
    WorldAction(legion::world::Event)
}