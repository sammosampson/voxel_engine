use voxel_engine_app::math;
use voxel_engine_app::terrain;
use super::chunk;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct ChunkBlockPosition {
    pub x: usize,
    pub y: usize,
    pub z: usize
}

impl ChunkBlockPosition {
    fn centre() -> Self {
        Self {
            x: terrain::BLOCKS_PER_CHUNK_X / 2,
            y: terrain::BLOCKS_PER_CHUNK_Y / 2,
            z: terrain::BLOCKS_PER_CHUNK_Z / 2,
        }
    }

    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self {
            x, y, z
        }
    }
    
    pub fn block_index(&self) -> usize {
        (self.z * terrain::BLOCKS_PER_CHUNK_Y) + (self.y * terrain::BLOCKS_PER_CHUNK_X) + self.x
    }
    
    pub fn absolute_centre(&self) -> math::Vector4 {
        let relative_centre = math::Vector4::position(terrain::BLOCK_SIZE, terrain::BLOCK_SIZE, terrain::BLOCK_SIZE) / 2.0;
        let centre: math::Vector4 = Self::centre().into();
        let current: math::Vector4 = self.clone().into();
        (current - centre) + relative_centre
    }
}


#[test]
fn position_has_correct_abs_centres() {
    assert_eq!(ChunkBlockPosition::default().absolute_centre(), math::Vector4::position(-7.5, -7.5, -7.5));
    assert_eq!(ChunkBlockPosition::new(terrain::BLOCKS_PER_CHUNK_X - 1, 0, 0).absolute_centre(), math::Vector4::position(7.5, -7.5, -7.5));
}

impl Into<math::Vector4> for ChunkBlockPosition {
    fn into(self) -> math::Vector4 {
        math::Vector4::position(self.x as f32, self.y as f32, self.z as f32)
    }
}

#[derive(Default)]
pub struct ChunkBlockPositionIterator {
    current: ChunkBlockPosition,
}

impl Iterator for ChunkBlockPositionIterator {
    type Item = ChunkBlockPosition;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        if self.current.x == terrain::BLOCKS_PER_CHUNK_X - 1 {
            if self.current.y == terrain::BLOCKS_PER_CHUNK_Y - 1 {
                if self.current.z == terrain::BLOCKS_PER_CHUNK_Z - 1 {
                    return None
                } else {
                    self.current = ChunkBlockPosition::new(0, 0, self.current.z + 1);
                }
            }  else {
                self.current = ChunkBlockPosition::new(0, self.current.y + 1, self.current.z);
            }
        } else {
            self.current = ChunkBlockPosition::new(self.current.x + 1, self.current.y, self.current.z);
        }

        Some(current)
    }
}

#[derive(Clone, Copy)]
pub struct Block {
    pub block_type: terrain::BlockType,
    position: ChunkBlockPosition
}

impl Block {
    pub fn new(chunk: &terrain::Chunk, position: ChunkBlockPosition) -> Self {
        Self {
            block_type: chunk::get_block(chunk, position),
            position
        }
    }

    pub fn absolute_position(&self) -> math::Vector4 {
        self.position.absolute_centre()
    }
}