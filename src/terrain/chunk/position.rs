use crate::prelude::*;

#[derive(Debug)]
pub struct ChunkPosition {
    x: i64,
    z: i64,
}

impl ChunkPosition {
    pub fn new(x: i64, z: i64) -> Self {
        Self {
            x,
            z
        }
    }

    pub fn absolute_centre(&self) -> Vector4 {
        Vector4::position(
            self.x as f32 * (COLUMNS_PER_CHUNK_X as f32 * BLOCK_SIZE),
            0.0,
            self.z as f32 * (COLUMNS_PER_CHUNK_Z as f32 * BLOCK_SIZE),
        )
    }
}
