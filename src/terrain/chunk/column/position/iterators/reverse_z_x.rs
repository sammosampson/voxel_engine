use crate::prelude::*;
    
#[derive(Default)]
pub struct ReverseZXColumnPositionIterator {
    current: Option<ColumnPosition>,
}

impl Iterator for ReverseZXColumnPositionIterator {
    type Item = ColumnPosition;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current {
            if current.z == 0 {
                if current.x == COLUMNS_PER_CHUNK_X - 1 {
                    return None
                } else {
                    self.current = Some(ColumnPosition::new(current.x + 1, COLUMNS_PER_CHUNK_Z - 1));
                }
            } else {
                self.current = Some(ColumnPosition::new(current.x, current.z - 1));
            }
        }
        else {
            self.current = Some(ColumnPosition::new(0, COLUMNS_PER_CHUNK_Z - 1));
        }

        self.current
    }
}

impl MainDimenisionMinimumCheck for ReverseZXColumnPositionIterator {
    fn at_main_dimension_minimum(&self) -> bool {
        if let Some(current) = self.current {
            return current.z == COLUMNS_PER_CHUNK_X - 1;
        }
        false
    }
}