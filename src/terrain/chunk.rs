use super::block;
use crate::math;
 

const CHUNK_DIMENSIONS:usize = super::COLUMNS_PER_CHUNK_Z * super::COLUMNS_PER_CHUNK_X;

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

    pub fn absolute_centre(&self) -> math::Vector4 {
        math::Vector4::position(
            self.x as f32 * (super::COLUMNS_PER_CHUNK_X as f32 * super::BLOCK_SIZE),
            0.0,
            self.z as f32 * (super::COLUMNS_PER_CHUNK_Z as f32 * super::BLOCK_SIZE),
        )
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct ColumnPosition {
    pub x: usize,
    pub z: usize
}

impl ColumnPosition {
    pub fn new(x: usize, z: usize) -> Self {
        Self {
            x, z
        }
    }

    fn x(&self) -> usize {
        self.x
    }

    fn z(&self) -> usize {
        self.z
    }

    fn chunk_index(&self) -> usize {
        (self.z * super::COLUMNS_PER_CHUNK_X) + self.x
    }
}

impl From<ColumnPosition> for block::BlockPosition {
    fn from(position: ColumnPosition) -> Self {
        Self::new(position.x, 0, position.z)
    }
}


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ColumnHeight {
    value: u16
}

impl From<u16> for ColumnHeight {
    fn from(value: u16) -> Self {
        Self {
            value
        }
    }
}

impl From<ColumnHeight> for u16 {
    fn from(height: ColumnHeight) -> Self {
        height.value
    }
}

impl From<ColumnHeight> for usize {
    fn from(height: ColumnHeight) -> Self {
        height.value as usize
    }
}


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

    pub fn top_most_blocks<'a>(&'a self) -> Box<dyn Iterator<Item=block::Block> + 'a>  {
        Box::new(TopMostBlockIterator::<'a>::new(self))
    }

    pub fn left_most_blocks<'a>(&'a self) -> Box<dyn Iterator<Item=block::Block> + 'a>   {
        Box::new(XZOuterMostBlockIterator::<'a, XZColumnPositionIterator>::x(self))
    }

    pub fn right_most_blocks<'a>(&'a self) -> Box<dyn Iterator<Item=block::Block> + 'a>   {
        Box::new(XZOuterMostBlockIterator::<'a, ReverseXZColumnPositionIterator>::x(self))
    }

    pub fn front_most_blocks<'a>(&'a self) -> Box<dyn Iterator<Item=block::Block> + 'a>   {
        Box::new(XZOuterMostBlockIterator::<'a, ZXColumnPositionIterator>::z(self))
    }

    pub fn back_most_blocks<'a>(&'a self) -> Box<dyn Iterator<Item=block::Block> + 'a>   {
        Box::new(XZOuterMostBlockIterator::<'a, ReverseZXColumnPositionIterator>::z(self))
    }
}
    
#[derive(Default)]
pub struct XZColumnPositionIterator {
    current: ColumnPosition,
}

impl Iterator for XZColumnPositionIterator {
    type Item = ColumnPosition;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        if self.current.x == super::COLUMNS_PER_CHUNK_X - 1 {
            if self.current.z == super::COLUMNS_PER_CHUNK_Z - 1 {
                return None
            } else {
                self.current = ColumnPosition::new(0, self.current.z + 1);
            }
        } else {
            self.current = ColumnPosition::new(self.current.x + 1, self.current.z);
        }

        Some(current)
    }
}

pub struct ReverseXZColumnPositionIterator {
    current: ColumnPosition,
}

impl Default for ReverseXZColumnPositionIterator {
    fn default() -> Self {
        Self { current: ColumnPosition::new(super::COLUMNS_PER_CHUNK_X - 1, 0) }
    }
}

impl Iterator for ReverseXZColumnPositionIterator {
    type Item = ColumnPosition;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        if self.current.x == 0 {
            if self.current.z == super::COLUMNS_PER_CHUNK_Z - 1 {
                return None
            } else {
                self.current = ColumnPosition::new(0, self.current.z + 1);
            }
        } else {
            self.current = ColumnPosition::new(self.current.x - 1, self.current.z);
        }

        Some(current)
    }
}

#[derive(Default)]
pub struct ZXColumnPositionIterator {
    current: ColumnPosition,
}

impl Iterator for ZXColumnPositionIterator {
    type Item = ColumnPosition;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        if self.current.z == super::COLUMNS_PER_CHUNK_Z - 1 {
            if self.current.x == super::COLUMNS_PER_CHUNK_X - 1 {
                return None
            } else {
                self.current = ColumnPosition::new(self.current.x + 1, 0);
            }
        } else {
            self.current = ColumnPosition::new(self.current.x, self.current.z + 1);
        }

        Some(current)
    }
}

pub struct ReverseZXColumnPositionIterator {
    current: ColumnPosition,
}

impl Default for ReverseZXColumnPositionIterator {
    fn default() -> Self {
        Self { current: ColumnPosition::new(0, super::COLUMNS_PER_CHUNK_Z - 1) }
    }
}

impl Iterator for ReverseZXColumnPositionIterator {
    type Item = ColumnPosition;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        if self.current.z == 0 {
            if self.current.x == super::COLUMNS_PER_CHUNK_X - 1 {
                return None
            } else {
                self.current = ColumnPosition::new(self.current.x + 1, 0);
            }
        } else {
            self.current = ColumnPosition::new(self.current.x, self.current.z - 1);
        }

        Some(current)
    }
}

pub struct TopMostBlockIterator<'a> {
    chunk: &'a Chunk,
    current_position: XZColumnPositionIterator
}

impl<'a> TopMostBlockIterator<'a> {
    fn new(chunk: &'a Chunk) -> Self {
        Self {
            chunk,
            current_position: XZColumnPositionIterator::default()
        }
    }
}

impl<'a> Iterator for TopMostBlockIterator<'a> {
    type Item = block::Block;

    fn next(&mut self) -> Option<Self::Item> {
        let column_position = self.current_position.next();
        if let Some(column_position) = column_position {
            let height = self.chunk.get_column(column_position);
            let block_position = block::BlockPosition::new(
                column_position.x(), 
                height.into(), 
                column_position.z()
            );
            return Some(block_position.into())
        }
        None
    }
}

pub struct XZOuterMostBlockIterator<'a, TColumnPositionIterator> 
where TColumnPositionIterator: Iterator<Item=ColumnPosition> + Default {
    on_x_axis: bool,
    chunk: &'a Chunk,
    current_column_position: TColumnPositionIterator,
    current_block_position: block::BlockPosition,
    current_height_watermark: usize
}

impl<'a, TColumnPositionIterator> XZOuterMostBlockIterator<'a, TColumnPositionIterator> 
where TColumnPositionIterator: Iterator<Item=ColumnPosition> + Default {
    fn x(chunk: &'a Chunk) -> Self {
        Self {
            on_x_axis: true,
            chunk,
            current_column_position: TColumnPositionIterator::default(),
            current_block_position: block::BlockPosition::default(),
            current_height_watermark: 0
        }
    }

    fn z(chunk: &'a Chunk) -> Self {
        Self {
            on_x_axis: false,
            chunk,
            current_column_position: TColumnPositionIterator::default(),
            current_block_position: block::BlockPosition::default(),
            current_height_watermark: 0
        }
    }
}

impl<'a, TColumnPositionIterator> Iterator for XZOuterMostBlockIterator<'a, TColumnPositionIterator> 
where TColumnPositionIterator: Iterator<Item=ColumnPosition> + Default {
    type Item = block::Block;

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
                        self.current_block_position = block::BlockPosition::from(column_position);
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
