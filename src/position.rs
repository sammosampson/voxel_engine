
use crate::prelude::*;

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct LastPosition(pub Vector4);

impl From<Position> for LastPosition {
    fn from(value: Position) -> Self {
        Self(value.0)
    }
}

impl Deref for LastPosition {
    type Target = Vector4;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Position(pub Vector4);

impl Deref for Position {
    type Target = Vector4;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vector4> for Position {
    fn from(value: Vector4) -> Self {
        Self(value)
    }
}

impl From<&Position> for Vector4 {
    fn from(position: &Position) -> Self {
        position.0
    }
}

impl Add<Vector4> for Position {
    type Output = Position;

    fn add(self, rhs: Vector4) -> Self::Output {
        Position(rhs + self.0)
    }
}
