mod top;
mod back;
mod front;
mod left;
mod right;

pub use top::*;
pub use back::*;
pub use front::*;
pub use left::*;
pub use right::*;

use crate::prelude::*;

pub trait Tesselate {
    fn tesselate(&mut self, chunk: &Chunk) {
        for block in self.get_outermost_blocks(chunk) {
            self.add_face(block.world_centre(), block.block_type().into())
        }    
    }

    fn get_outermost_blocks<'a>(&mut self, chunk: &'a Chunk) -> Box<dyn Iterator<Item=Block> +'a>;

    fn add_face(&mut self, centre: Vector4, colour: Colour);
}

pub const FACE_SIZE:f32 = 1.0;