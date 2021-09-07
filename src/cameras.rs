use crate::math;

#[derive(Clone, Copy, PartialEq)]
pub struct Camera {
    pub position: math::Vector4,
    pub direction: math::Vector4,
    pub up: math::Vector4
}

#[derive(Clone, Copy, PartialEq)]
pub struct AttachCamera {
    pub offset_position: math::Vector4,
    pub offset_direction: math::Vector4
}