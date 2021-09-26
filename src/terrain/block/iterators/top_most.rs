use crate::prelude::*;

pub struct TopMostBlockIterator<'a> {
    chunk: &'a Chunk,
    current_position: XZColumnPositionIterator
}

impl<'a> TopMostBlockIterator<'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        Self {
            chunk,
            current_position: XZColumnPositionIterator::default()
        }
    }
}

impl<'a> Iterator for TopMostBlockIterator<'a> {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        let column_position = self.current_position.next();
        if let Some(column_position) = column_position {
            let height = self.chunk.get_column(column_position);
            let block_position = BlockPosition::new(
                column_position.x(), 
                height.into(), 
                column_position.z()
            );
            return Some(block_position.into())
        }
        None
    }
}
