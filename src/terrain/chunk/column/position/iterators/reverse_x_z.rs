use crate::prelude::*;

pub struct ReverseXZColumnPositionIterator {
    current: ColumnPosition,
}

impl Default for ReverseXZColumnPositionIterator {
    fn default() -> Self {
        Self { current: ColumnPosition::new(COLUMNS_PER_CHUNK_X - 1, 0) }
    }
}

impl Iterator for ReverseXZColumnPositionIterator {
    type Item = ColumnPosition;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        if self.current.x == 0 {
            if self.current.z == COLUMNS_PER_CHUNK_Z - 1 {
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

