use crate::prelude::*;
    
#[derive(Default)]
pub struct ZXColumnPositionIterator {
    current: ColumnPosition,
}

impl Iterator for ZXColumnPositionIterator {
    type Item = ColumnPosition;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        if self.current.z == COLUMNS_PER_CHUNK_Z - 1 {
            if self.current.x == COLUMNS_PER_CHUNK_X - 1 {
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