use crate::mesh;
use crate::math;

pub const BLOCK_SIZE:f32 = 1.0;
pub const BLOCKS_PER_CHUNK_X:usize = 16;
pub const BLOCKS_PER_CHUNK_Y:usize = 16;
pub const BLOCKS_PER_CHUNK_Z:usize = 16;

#[derive(Debug, Clone, Copy)]
pub enum BlockType {
    Air,
    Grass,
    Brick
}

const CHUNK_DIMENSIONS:usize =
    BLOCKS_PER_CHUNK_Z
    * BLOCKS_PER_CHUNK_Y
    * BLOCKS_PER_CHUNK_X;

#[derive(Debug)]
pub struct Chunk {
    pub blocks: [BlockType; CHUNK_DIMENSIONS]
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
            self.x as f32 * (BLOCKS_PER_CHUNK_X as f32 * BLOCK_SIZE),
            self.y as f32 * (BLOCKS_PER_CHUNK_Y as f32 * BLOCK_SIZE),
            self.z as f32 * (BLOCKS_PER_CHUNK_Z as f32 * BLOCK_SIZE),
        )
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            blocks: [BlockType::Air; CHUNK_DIMENSIONS]
        }
    }
}

#[derive(Debug)]
pub struct ChunkPosition {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, Default, Clone)]
pub struct ChunkFrontMesh {
    pub vertices: Vec<mesh::Vertex>
}

#[derive(Debug, Default, Clone)]
pub struct ChunkBackMesh {
    pub vertices: Vec<mesh::Vertex>
}

#[derive(Debug, Default, Clone)]
pub struct ChunkTopMesh {
    pub vertices: Vec<mesh::Vertex>
}

#[derive(Debug, Default, Clone)]
pub struct ChunkBottomMesh {
    pub vertices: Vec<mesh::Vertex>
}

#[derive(Debug, Default, Clone)]
pub struct ChunkRightMesh {
    pub vertices: Vec<mesh::Vertex>
}

#[derive(Debug, Default, Clone)]
pub struct ChunkLeftMesh {
    pub vertices: Vec<mesh::Vertex>
}
