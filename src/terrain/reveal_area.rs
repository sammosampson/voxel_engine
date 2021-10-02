
use crate::prelude::*;
pub struct TerrainRevealAreaSize(pub u8);

pub struct TerrainRevealArea {
    centre_position: ChunkPosition,
    size: u8,
    current: Option<ChunkPosition>
}

struct TerrainRevealAreaIterator(TerrainRevealArea);

impl TerrainRevealArea {
    pub fn new(radius: TerrainRevealAreaSize, centre_position: Position) -> Self {
        Self {
            centre_position: ChunkPosition::from(centre_position),
            size: radius.0,
            current: None
        }
    }

    pub fn iter(self) -> TerrainRevealAreaIterator {
        TerrainRevealAreaIterator(self)
    }
}

impl Iterator for TerrainRevealAreaIterator {
    type Item = ChunkPosition;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.0.current {
            self.0.current = Some(centre_position - ChunkPosition::new())
            return None;
        }  
        None
    }
}