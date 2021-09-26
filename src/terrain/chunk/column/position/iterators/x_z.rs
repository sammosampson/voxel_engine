use crate::prelude::*;
    
#[derive(Default)]
pub struct XZColumnPositionIterator {
    current: ColumnPosition,
}

impl Iterator for XZColumnPositionIterator {
    type Item = ColumnPosition;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        if self.current.x == COLUMNS_PER_CHUNK_X - 1 {
            if self.current.z == COLUMNS_PER_CHUNK_Z - 1 {
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