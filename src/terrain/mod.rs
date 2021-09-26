
mod chunk;
mod mesh;
mod block;
pub use mesh::*;
pub use block::*;
pub use chunk::*;

pub const BLOCK_SIZE:f32 = 1.0;
pub const COLUMNS_PER_CHUNK_X:usize = 16;
pub const COLUMNS_PER_CHUNK_Z:usize = 16;

pub struct TerrainRevealRadius(pub u8);

