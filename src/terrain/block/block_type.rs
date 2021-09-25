use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum BlockType {
    Sea,
    Sand,
    Grass,
    Rock,
    Ice,
    Air,
}

impl From<BlockType> for Colour {
    fn from(block_type: BlockType) -> Self {
        match block_type {
            BlockType::Sea => Colour::blue(),
            BlockType::Sand => Colour::yellow(),
            BlockType::Grass => Colour::green(),
            BlockType::Rock => Colour::grey(),
            BlockType::Ice => Colour::white(),
            BlockType::Air => Colour::black(),
        }
    }
}
