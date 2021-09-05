
mod chunk;
mod mesh;
mod block;
pub use mesh::*;
pub use block::*;
pub use chunk::*;

pub const BLOCK_SIZE:f32 = 1.0;
pub const BLOCKS_PER_CHUNK_X:usize = 16;
pub const BLOCKS_PER_CHUNK_Y:usize = 16;
pub const BLOCKS_PER_CHUNK_Z:usize = 16;

pub fn full_chunk_shape() -> chunk::Chunk {
    let mut chunk = chunk::Chunk::default();
    let positions = block::ChunkBlockPositionIterator::default();
    
    for block_position in positions {
        if block_position.x % 2 == 0 {
            chunk::place_block(&mut chunk, block_position, block::BlockType::Brick)
        }
        else {
            chunk::place_block(&mut chunk, block_position, block::BlockType::Grass)
        }
    }
    chunk
}
