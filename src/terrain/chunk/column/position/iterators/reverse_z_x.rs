use crate::prelude::*;
    
pub struct ReverseZXColumnPositionIterator {
    current: ColumnPosition,
}

impl Default for ReverseZXColumnPositionIterator {
    fn default() -> Self {
        Self { current: ColumnPosition::new(0, COLUMNS_PER_CHUNK_Z - 1) }
    }
}

impl Iterator for ReverseZXColumnPositionIterator {
    type Item = ColumnPosition;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        if self.current.z == 0 {
            if self.current.x == COLUMNS_PER_CHUNK_X - 1 {
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