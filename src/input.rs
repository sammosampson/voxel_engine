use crate::math;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputState {
    None,
    Pressed,
    Released
}

impl Default for InputState {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MouseButton {
    None,
    Left,
    Middle,
    Right
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeyboardButton {
    pub key_code: Option<glium::glutin::event::VirtualKeyCode>
}

impl KeyboardButton {
    pub fn is_pressed(&self, key_code: glium::glutin::event::VirtualKeyCode, state: &InputState) -> bool {
        if state != &InputState::Pressed {
            return false;
        } 

        if let Some(code) = self.key_code {
            return code == key_code
        }

        false
    }
}

impl Default for MouseButton {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct MouseAction {
    pub position: math::Vector2,
    pub button: MouseButton,
    pub state: InputState,
    pub wheel_movement: math::Vector2
}

impl MouseAction {
    pub fn left_button_pressed(&self) -> bool {
        self.button == MouseButton::Left 
            && self.state == InputState::Pressed
    }
    
    pub fn middle_button_pressed(&self) -> bool {
        self.button == MouseButton::Middle 
            && self.state == InputState::Pressed
    }

    pub fn middle_button_pressed_or_released(&self) -> bool {
        self.button == MouseButton::Middle 
            && (self.state == InputState::Pressed || self.state == InputState::Released)
    }

    pub fn right_button_pressed(&self) -> bool {
        self.button == MouseButton::Right 
            && self.state == InputState::Pressed
    }

    pub fn right_button_pressed_or_released(&self) -> bool {
        self.button == MouseButton::Right 
            && (self.state == InputState::Pressed || self.state == InputState::Released)
    }

    pub fn mouse_wheel_moved(&self) -> bool {
        !self.wheel_movement.zeroed()
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct MouseInput {
    pub last_previous_action: MouseAction,
    pub current_actions: Vec<MouseAction>
}
