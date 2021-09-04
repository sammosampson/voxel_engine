
mod chunk;
mod mesh;
mod block;
pub use mesh::*;

use voxel_engine_app::terrain;

pub fn wierd_shape() -> terrain::Chunk {
    let mut chunk = terrain::Chunk::default();
    let positions = block::ChunkBlockPositionIterator::default();
    
    for block_position in positions {
        if (block_position.absolute_centre()).length().abs() >= 7.5 {
            continue;
        }
        if block_position.x % 2 == 0 {
            chunk::place_block(&mut chunk, block_position, terrain::BlockType::Brick)
        }
        else {
            chunk::place_block(&mut chunk, block_position, terrain::BlockType::Grass)
        }
    }
    chunk
}
  