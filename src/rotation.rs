
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConstantRotation {
    pub x_revoloutions_per_second: f32,
    pub y_revoloutions_per_second: f32
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Rotation {
    pub x: f32,
    pub y: f32
}
