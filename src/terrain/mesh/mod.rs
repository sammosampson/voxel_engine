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

const FACE_SIZE:f32 = 1.0;

#[derive(Debug, Default, Clone)]
pub struct ChunkFrontMesh {
    pub vertices: Vec<Vertex>
}

#[derive(Debug, Default, Clone)]
pub struct ChunkBackMesh {
    pub vertices: Vec<Vertex>
}

#[derive(Debug, Default, Clone)]
pub struct ChunkTopMesh {
    pub vertices: Vec<Vertex>
}

#[derive(Debug, Default, Clone)]
pub struct ChunkRightMesh {
    pub vertices: Vec<Vertex>
}

#[derive(Debug, Default, Clone)]
pub struct ChunkLeftMesh {
    pub vertices: Vec<Vertex>
}

impl Tesselate for ChunkFrontMesh {
    fn add_face(&mut self, centre: Vector4, colour: Colour) {
        CuboidMeshFace::front(centre, FACE_SIZE).tesselate(&mut self.vertices, colour);
    }

    fn get_outermost_blocks<'a>(&mut self, chunk: &'a Chunk) -> Box<dyn Iterator<Item=Block> +'a> {
        chunk.front_most_blocks()
    }
}

impl Tesselate for ChunkBackMesh {
    fn add_face(&mut self, centre: Vector4, colour: Colour) {
        CuboidMeshFace::back(centre, FACE_SIZE).tesselate(&mut self.vertices, colour);
    }

    fn get_outermost_blocks<'a>(&mut self, chunk: &'a Chunk) -> Box<dyn Iterator<Item=Block> +'a> {
        chunk.back_most_blocks()
    }
}

impl Tesselate for ChunkTopMesh {
    fn add_face(&mut self, centre: Vector4, colour: Colour) {
        CuboidMeshFace::top(centre, FACE_SIZE).tesselate(&mut self.vertices, colour);
    }

    fn get_outermost_blocks<'a>(&mut self, chunk: &'a Chunk) -> Box<dyn Iterator<Item=Block> +'a> {
        chunk.top_most_blocks()
    }
}
impl Tesselate for ChunkLeftMesh {
    fn add_face(&mut self, centre: Vector4, colour: Colour) {
        CuboidMeshFace::left(centre, FACE_SIZE).tesselate(&mut self.vertices, colour);
    }

    fn get_outermost_blocks<'a>(&mut self, chunk: &'a Chunk) -> Box<dyn Iterator<Item=Block> +'a> {
        chunk.left_most_blocks()
    }
}

impl Tesselate for ChunkRightMesh {
    fn add_face(&mut self, centre: Vector4, colour: Colour) {
        CuboidMeshFace::right(centre, FACE_SIZE).tesselate(&mut self.vertices, colour);
    }

    fn get_outermost_blocks<'a>(&mut self, chunk: &'a Chunk) -> Box<dyn Iterator<Item=Block> +'a> {
        chunk.right_most_blocks()
    }
}