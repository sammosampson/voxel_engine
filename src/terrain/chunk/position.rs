use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPosition {
    pub x: i64,
    pub z: i64,
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

impl From<ChunkPosition> for WorldEntityId {
    fn from(position: ChunkPosition) -> Self {
        Self {
            name: format!("chunk{:?}{:?}", position.x, position.z)
        }
    }
}

impl From<Vector4> for ChunkPosition {
    fn from(position: Vector4) -> Self {
        Self {
            x: (position.x / (COLUMNS_PER_CHUNK_X as f32 * BLOCK_SIZE)) as i64,
            z: (position.z / (COLUMNS_PER_CHUNK_Z as f32 * BLOCK_SIZE)) as i64
        }
    }
}

impl Add for ChunkPosition {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            z: self.z + rhs.z
        }
    }
}

impl Sub for ChunkPosition {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            z: self.z - rhs.z
        }
    }
}

