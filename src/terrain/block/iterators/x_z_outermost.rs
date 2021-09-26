use crate::prelude::*;

pub struct OuterMostBlockIterator<'a, TColumnPositionIterator> 
where TColumnPositionIterator: Iterator<Item=ColumnPosition> + Default {
    chunk: &'a Chunk,
    current_column_position: TColumnPositionIterator,
    current_block_position: BlockPosition,
    current_height_watermark: usize
}

impl<'a, TColumnPositionIterator> OuterMostBlockIterator<'a, TColumnPositionIterator> 
where TColumnPositionIterator: Iterator<Item=ColumnPosition> + MainDimenisionMinimumCheck + Default {
    pub fn new(chunk: &'a Chunk) -> Self {
        Self {
            chunk,
            current_column_position: TColumnPositionIterator::default(),
            current_block_position: BlockPosition::default(),
            current_height_watermark: 0
        }
    }
}

impl<'a, TColumnPositionIterator> Iterator for OuterMostBlockIterator<'a, TColumnPositionIterator> 
where TColumnPositionIterator: Iterator<Item=ColumnPosition> + MainDimenisionMinimumCheck + Default {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {    
        if self.current_block_position.y == self.current_height_watermark {
            loop {
                let column_position = self.current_column_position.next();
                if let Some(column_position) = column_position {
                    if self.current_column_position.at_main_dimension_minimum() {
                        self.current_height_watermark = 0;
                    }
                    
                    let height = self.chunk.get_column(column_position).into();

                    if height > self.current_height_watermark {
                        self.current_block_position = BlockPosition::from(column_position);
                        self.current_height_watermark = height;
                        break;
                    }
                } else {
                    return None;
                }
            }
        }
                    
        self.current_block_position = self.current_block_position.increase_height();
        Some(self.current_block_position.into())
    }
}