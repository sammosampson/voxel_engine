#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Size2d {
    pub width: f32,
    pub height: f32
}

impl Size2d {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height
        }
    }
}