use crate::math;
use crate::rendering;


#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: math::Vector4,
    pub normal: math::Vector4,
    pub colour: rendering::Colour,
    pub uv: math::Vector2
}

impl Vertex {
    pub fn new(position: math::Vector4, normal: math::Vector4, colour: rendering::Colour, uv: math::Vector2) -> Self {
        Self {
            position,
            normal,
            colour,
            uv
        }
    }
}

#[derive(Debug)]
pub struct Mesh {
    pub data: Vec<Vertex>,
}

impl Mesh {
    pub fn new(data: Vec<Vertex>) -> Self {
        Self {
            data
        }
    }
}