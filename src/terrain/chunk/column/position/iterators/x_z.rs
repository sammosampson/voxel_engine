use crate::prelude::*;
    
#[derive(Default)]
pub struct XZColumnPositionIterator {
    current: Option<ColumnPosition>,
}

impl Iterator for XZColumnPositionIterator {
    type Item = ColumnPosition;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current {
            if current.x == COLUMNS_PER_CHUNK_X - 1 {
                if current.z == COLUMNS_PER_CHUNK_Z - 1 {
                    return None
                } else {
                    self.current = Some(ColumnPosition::new(0, current.z + 1));
                }
            } else {
                self.current = Some(ColumnPosition::new(current.x + 1, current.z));
            }
        }
        else {
            self.current = Some(ColumnPosition::default());
        }

        self.current
    }
}

impl MainDimenisionMinimumCheck for XZColumnPositionIterator {
    fn at_main_dimension_minimum(&self) -> bool {
        if let Some(current) = self.current {
            return current.x == 0;
        }
        false
    }
}