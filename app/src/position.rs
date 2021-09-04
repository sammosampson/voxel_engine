use crate::math;

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub value: math::Vector4
}

impl From<math::Vector4> for Position {
    fn from(value: math::Vector4) -> Self {
        Self {
            value
        }
    }
}
