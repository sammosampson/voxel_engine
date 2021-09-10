use crate::terrain;
use crate::math;

#[derive(Debug, Clone, Copy)]
pub enum BlockType {
    Sea,
    Sand,
    Grass,
    Rock,
    Ice,
    Air,
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct BlockPosition {
    pub x: usize,
    pub y: usize,
    pub z: usize
}

impl BlockPosition {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self {
            x, y, z
        }
    }
    pub fn increase_height(&self) -> Self {
        Self {
            x: self.x, 
            y: self.y + 1,
            z: self.z
        }
    }

    fn centre() -> Self {
        Self {
            x: terrain::COLUMNS_PER_CHUNK_X / 2,
            y: 0,
            z: terrain::COLUMNS_PER_CHUNK_Z / 2,
        }
    }
    
    pub fn world_centre(&self) -> math::Vector4 {
        let relative_centre = math::Vector4::position(terrain::BLOCK_SIZE, terrain::BLOCK_SIZE, terrain::BLOCK_SIZE) / 2.0;
        let centre: math::Vector4 = Self::centre().into();
        let current: math::Vector4 = self.clone().into();
        (current - centre) + relative_centre
    }
}

impl From<BlockPosition> for math::Vector4 {
    fn from(position: BlockPosition) -> Self {
        Self::position(position.x as f32, position.y as f32, position.z as f32)
    }
}

impl From<BlockPosition> for BlockType {
    fn from(position: BlockPosition) -> Self {
        match position.y {
            0..=5 => BlockType::Sea,
            6..=10 => BlockType::Sand,
            11..=500 => BlockType::Grass,
            501..=900 => BlockType::Rock,
            901..=1000 => BlockType::Ice,
            _ => BlockType::Air,
        }
    }
}

pub struct Block {
    position: BlockPosition
}

impl Block {
    pub fn world_centre(&self) -> math::Vector4 {
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