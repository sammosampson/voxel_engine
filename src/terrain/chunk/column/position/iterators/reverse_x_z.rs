use crate::prelude::*;

#[derive(Default)]
pub struct ReverseXZColumnPositionIterator {
    current: Option<ColumnPosition>,
}

impl Iterator for ReverseXZColumnPositionIterator {
    type Item = ColumnPosition;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current {
            if current.x == 0 {
                if current.z == COLUMNS_PER_CHUNK_Z - 1 {
                    return None
                } else {
                    self.current = Some(ColumnPosition::new(COLUMNS_PER_CHUNK_X - 1, current.z + 1));
                }
            } else {
                self.current = Some(ColumnPosition::new(current.x - 1, current.z));
            }
        }
        else {
            self.current = Some(ColumnPosition::new(COLUMNS_PER_CHUNK_X - 1, 0));
        }

        self.current
    }
}

impl MainDimenisionMinimumCheck for ReverseXZColumnPositionIterator {
    fn at_main_dimension_minimum(&self) -> bool {
        if let Some(current) = self.current {
            return current.x == COLUMNS_PER_CHUNK_X - 1;
        }
        false
    }
}

