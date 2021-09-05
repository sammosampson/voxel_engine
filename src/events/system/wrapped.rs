use crate::input::KeyboardButton;
use crate::math;
use crate::input;
use crate::events;

pub struct WrappedSystemEvent(events::SystemEvent);

impl WrappedSystemEvent {
    pub fn new(wrapped: events::SystemEvent) -> Self {
        Self(wrapped)
    }
}

impl Into<events::SystemEvent> for WrappedSystemEvent {
    fn into(self) -> events::SystemEvent {
        self.0
    }
}

#[allow(deprecated)]
impl From<glium::glutin::event::WindowEvent<'_>> for WrappedSystemEvent {
    fn from(event: glium::glutin::event::WindowEvent) -> Self {
        match event {
            glium::glutin::event:: WindowEvent::Resized(size) => WrappedSystemEvent::new(events::SystemEvent::Resized(size.into())),
            glium::glutin::event:: WindowEvent::CloseRequested => WrappedSystemEvent::new(events::SystemEvent::CloseRequested),
            glium::glutin::event::WindowEvent::MouseInput { device_id: _, state, button, modifiers: _ } => 
                WrappedSystemEvent::new(events::SystemEvent::MouseButtonAction { state: state.into(), button: button.into() }),
            glium::glutin::event::WindowEvent::MouseWheel { device_id: _, delta, phase: _, modifiers: _} => 
                WrappedSystemEvent::new(events::SystemEvent::MouseWheelAction { delta: delta.into() }),
            glium::glutin::event::WindowEvent::CursorMoved { device_id: _, position, modifiers: _ } => 
                WrappedSystemEvent::new(events::SystemEvent::PointerMoved { position: position.into() }),
            glium::glutin::event::WindowEvent::KeyboardInput { device_id: _, input, is_synthetic: _ } => 
                WrappedSystemEvent::new(events::SystemEvent::KeyboardAction { state: input.state.into(), button: input.virtual_keycode.into() }),
            _ => WrappedSystemEvent::new(events::SystemEvent::Unused)
        }
    }
}

impl From<legion::world::Event> for WrappedSystemEvent {
    fn from(event: legion::world::Event) -> Self {
        WrappedSystemEvent::new(events::SystemEvent::WorldAction(event))
    }
}

impl Into<input::InputState> for glium::glutin::event::ElementState {
    fn into(self) -> input::InputState {
        match self {
            glium::glutin::event::ElementState::Pressed => input::InputState::Pressed,
            glium::glutin::event::ElementState::Released => input::InputState::Released,
        }
    }
}

impl Into<input::MouseButton> for glium::glutin::event::MouseButton {
    fn into(self) -> input::MouseButton {
        match self {
            glium::glutin::event::MouseButton::Left => input::MouseButton::Left,
            glium::glutin::event::MouseButton::Middle => input::MouseButton::Middle,
            glium::glutin::event::MouseButton::Right => input::MouseButton::Right,
            glium::glutin::event::MouseButton::Other(_) => input::MouseButton::None,
        }
    }
}

impl Into<KeyboardButton> for Option<glium::glutin::event::VirtualKeyCode> {
    fn into(self) -> KeyboardButton {
        KeyboardButton { key_code: self }
    }
}

impl Into<math::Vector2> for glium::glutin::dpi::PhysicalPosition<f64> {
    fn into(self) -> math::Vector2 {
        math::Vector2::new(self.x as f32, self.y as f32)
    }
}
 
impl Into<math::Size2d> for glium::glutin::dpi::PhysicalSize<u32> {
    fn into(self) -> math::Size2d {
        math::Size2d::new(self.width as f32, self.height as f32)
    }
}

impl Into<math::Vector2> for glium::glutin::event::MouseScrollDelta {
    fn into(self) -> math::Vector2 {
        match self {
            glium::glutin::event::MouseScrollDelta::LineDelta(x, y) => math::Vector2::new(x, y),
            glium::glutin::event::MouseScrollDelta::PixelDelta(_) => todo!(),
        }
    }
}
 
impl From<glium::glutin::event::DeviceEvent> for WrappedSystemEvent {
    fn from(_event: glium::glutin::event::DeviceEvent) -> Self {
        WrappedSystemEvent::new(events::SystemEvent::Unused)
    }
}

