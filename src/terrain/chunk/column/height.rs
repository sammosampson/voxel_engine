

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
