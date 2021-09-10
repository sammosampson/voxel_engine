
mod chunk;
mod mesh;
mod block;
pub use mesh::*;
pub use block::*;
pub use chunk::*;

pub const BLOCK_SIZE:f32 = 1.0;
pub const COLUMNS_PER_CHUNK_X:usize = 16;
pub const COLUMNS_PER_CHUNK_Z:usize = 16;

pub fn full_chunk_shape() -> chunk::Chunk {
    let mut chunk = chunk::Chunk::default();
    let column_positions = chunk.column_positions();
    
    for column_position in column_positions {
        chunk.place_column(column_position, 50.into());
    }

    chunk
}
