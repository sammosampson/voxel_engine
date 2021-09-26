use crate::prelude::*;

pub struct XZOuterMostBlockIterator<'a, TColumnPositionIterator> 
where TColumnPositionIterator: Iterator<Item=ColumnPosition> + Default {
    on_x_axis: bool,
    chunk: &'a Chunk,
    current_column_position: TColumnPositionIterator,
    current_block_position: BlockPosition,
    current_height_watermark: usize
}

impl<'a, TColumnPositionIterator> XZOuterMostBlockIterator<'a, TColumnPositionIterator> 
where TColumnPositionIterator: Iterator<Item=ColumnPosition> + Default {
    pub fn x(chunk: &'a Chunk) -> Self {
        Self {
            on_x_axis: true,
            chunk,
            current_column_position: TColumnPositionIterator::default(),
            current_block_position: BlockPosition::default(),
            current_height_watermark: 0
        }
    }

    pub fn z(chunk: &'a Chunk) -> Self {
        Self {
            on_x_axis: false,
            chunk,
            current_column_position: TColumnPositionIterator::default(),
            current_block_position: BlockPosition::default(),
            current_height_watermark: 0
        }
    }
}

impl<'a, TColumnPositionIterator> Iterator for XZOuterMostBlockIterator<'a, TColumnPositionIterator> 
where TColumnPositionIterator: Iterator<Item=ColumnPosition> + Default {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {    
        if self.current_block_position.y == self.current_height_watermark {
            loop {
                let column_position = self.current_column_position.next();
                if let Some(column_position) = column_position {
                    if (self.on_x_axis && column_position.x() == 0) 
                        || (!self.on_x_axis && column_position.z() == 0) {
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