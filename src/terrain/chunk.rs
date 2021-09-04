use super::block;
use crate::math;
 

const CHUNK_DIMENSIONS:usize =
    super::BLOCKS_PER_CHUNK_Z
    * super::BLOCKS_PER_CHUNK_Y
    * super::BLOCKS_PER_CHUNK_X;

#[derive(Debug)]
pub struct Chunk {
    pub blocks: [block::BlockType; CHUNK_DIMENSIONS]
}

impl ChunkPosition {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self {
            x,
            y,
            z
        }
    }

    pub fn absolute_centre(&self) -> math::Vector4 {
        math::Vector4::position(
            self.x as f32 * (super::BLOCKS_PER_CHUNK_X as f32 * super::BLOCK_SIZE),
            self.y as f32 * (super::BLOCKS_PER_CHUNK_Y as f32 * super::BLOCK_SIZE),
            self.z as f32 * (super::BLOCKS_PER_CHUNK_Z as f32 * super::BLOCK_SIZE),
        )
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            blocks: [block::BlockType::Air; CHUNK_DIMENSIONS]
        }
    }
}

#[derive(Debug)]
pub struct ChunkPosition {
    x: i64,
    y: i64,
    z: i64,
}

pub fn get_block(chunk: &Chunk, position: block::ChunkBlockPosition) -> block::BlockType {
    chunk.blocks[position.block_index()]
}

pub fn place_block(chunk: &mut Chunk, position: block::ChunkBlockPosition, block_type: block::BlockType) {
    chunk.blocks[position.block_index()] = block_type;
}

pub struct ChunkIterator<'a> {
    chunk: &'a Chunk,
    positions: block::ChunkBlockPositionIterator,
}

impl<'a> ChunkIterator<'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        Self {
            chunk, 
            positions: block::ChunkBlockPositionIterator::default()
        }
    }
}

impl<'a> Iterator for ChunkIterator<'a> {
    type Item = block::Block;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(position) = self.positions.next() {
            return Some(block::Block::new(self.chunk, position));
        }
        None
    }
}