mod position;
mod block_type;

pub use position::*;
pub use block_type::*;

use crate::prelude::*;

pub struct Block {
    position: BlockPosition
}

impl Block {
    pub fn world_centre(&self) -> Vector4 {
        self.position.world_centre()
    }

    pub fn block_type(&self) -> BlockType {
        self.position.into()
    }
}

impl From<BlockPosition> for Block {
    fn from(position: BlockPosition) -> Self {
        Self {
            position
        }
    }
}