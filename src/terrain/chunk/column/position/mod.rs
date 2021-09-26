mod iterators;
pub use iterators::*;

use crate::prelude::*;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct ColumnPosition {
    pub x: usize,
    pub z: usize
}

impl ColumnPosition {
    pub fn new(x: usize, z: usize) -> Self {
        Self {
            x, z
        }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn z(&self) -> usize {
        self.z
    }

    pub fn chunk_index(&self) -> usize {
        (self.z * COLUMNS_PER_CHUNK_X) + self.x
    }
}

impl From<ColumnPosition> for BlockPosition {
    fn from(position: ColumnPosition) -> Self {
        Self::new(position.x, 0, position.z)
    }
}