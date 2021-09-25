use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Vector4,
    pub normal: Vector4,
    pub colour: Colour,
    pub uv: Vector2
}

impl Vertex {
    pub fn new(position: Vector4, normal: Vector4, colour: Colour, uv: Vector2) -> Self {
        Self {
            position,
            normal,
            colour,
            uv
        }
    }
}
