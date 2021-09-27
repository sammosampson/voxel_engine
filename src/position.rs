use crate::math;

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Position(pub math::Vector4);

impl From<math::Vector4> for Position {
    fn from(value: math::Vector4) -> Self {
        Self(value)
    }
}

impl From<&Position> for math::Vector4 {
    fn from(position: &Position) -> Self {
        position.0
    }
}
