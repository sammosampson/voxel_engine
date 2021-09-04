use voxel_engine_app::terrain;
use super::block;
 
pub fn get_block(chunk: &terrain::Chunk, position: block::ChunkBlockPosition) -> terrain::BlockType {
    chunk.blocks[position.block_index()]
}

pub fn place_block(chunk: &mut terrain::Chunk, position: block::ChunkBlockPosition, block_type: terrain::BlockType) {
    chunk.blocks[position.block_index()] = block_type;
}

pub struct ChunkIterator<'a> {
    chunk: &'a terrain::Chunk,
    positions: block::ChunkBlockPositionIterator,
}

impl<'a> ChunkIterator<'a> {
    pub fn new(chunk: &'a terrain::Chunk) -> Self {
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