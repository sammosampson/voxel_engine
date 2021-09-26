mod position;
mod column;
pub use position::*;
pub use column::*;

use crate::prelude::*;
 
pub fn full_chunk_shape() -> Chunk {
    let mut chunk = Chunk::default();
    let column_positions = chunk.column_positions();
    
    for column_position in column_positions {
        chunk.place_column(column_position, 50.into());
    }

    chunk
}


const CHUNK_DIMENSIONS:usize = COLUMNS_PER_CHUNK_Z * COLUMNS_PER_CHUNK_X;


#[derive(Debug)]
pub struct Chunk {
    pub columns: [u16; CHUNK_DIMENSIONS]
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            columns: [0; CHUNK_DIMENSIONS]
        }
    }
}

impl Chunk {
    pub fn get_column(&self, position: ColumnPosition) -> ColumnHeight {
        self.columns[position.chunk_index()].into()
    }
    
    pub fn place_column(&mut self, position: ColumnPosition, height: ColumnHeight) {
        self.columns[position.chunk_index()] = height.into();
    }

    pub fn column_positions(&self) -> XZColumnPositionIterator {
        XZColumnPositionIterator::default()
    }

    pub fn top_most_blocks<'a>(&'a self) -> Box<dyn Iterator<Item=Block> + 'a>  {
        Box::new(TopMostBlockIterator::<'a>::new(self))
    }

    pub fn left_most_blocks<'a>(&'a self) -> Box<dyn Iterator<Item=Block> + 'a>   {
        Box::new(XZOuterMostBlockIterator::<'a, XZColumnPositionIterator>::x(self))
    }

    pub fn right_most_blocks<'a>(&'a self) -> Box<dyn Iterator<Item=Block> + 'a>   {
        Box::new(XZOuterMostBlockIterator::<'a, ReverseXZColumnPositionIterator>::x(self))
    }

    pub fn front_most_blocks<'a>(&'a self) -> Box<dyn Iterator<Item=Block> + 'a>   {
        Box::new(XZOuterMostBlockIterator::<'a, ZXColumnPositionIterator>::z(self))
    }

    pub fn back_most_blocks<'a>(&'a self) -> Box<dyn Iterator<Item=Block> + 'a>   {
        Box::new(XZOuterMostBlockIterator::<'a, ReverseZXColumnPositionIterator>::z(self))
    }
}
