use crate::prelude::*;
    
#[derive(Default)]
pub struct ZXColumnPositionIterator {
    current: Option<ColumnPosition>,
}

impl Iterator for ZXColumnPositionIterator {
    type Item = ColumnPosition;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current {
            if current.z == COLUMNS_PER_CHUNK_Z - 1 {
                if current.x == COLUMNS_PER_CHUNK_X - 1 {
                    return None
                } else {
                    self.current = Some(ColumnPosition::new(current.x + 1, 0));
                }
            } else {
                self.current = Some(ColumnPosition::new(current.x, current.z + 1));
            }
        }
        else {
            self.current = Some(ColumnPosition::default());
        }

        self.current
    }
}

impl MainDimenisionMinimumCheck for ZXColumnPositionIterator {
    fn at_main_dimension_minimum(&self) -> bool {
        if let Some(current) = self.current {
            return current.z == 0;
        }
        false
    }
}