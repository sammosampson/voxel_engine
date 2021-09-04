use crate::math;

#[derive(Clone, Copy, PartialEq)]
pub struct Camera {
    pub position: math::Vector4,
    pub direction: math::Vector4,
    pub up: math::Vector4
}