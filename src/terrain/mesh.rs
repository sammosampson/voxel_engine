use crate::mesh;
use crate::rendering;
use crate::math;
use super::chunk;
use super::block;


pub trait Tesselate {
    fn tesselate(&mut self, chunk: &chunk::Chunk) {
        for block in self.get_outermost_blocks(chunk) {
            self.add_face(block.world_centre(), block.block_type().into())
        }    
    }

    fn get_outermost_blocks<'a>(&mut self, chunk: &'a chunk::Chunk) -> Box<dyn Iterator<Item=block::Block> +'a>;

    fn add_face(&mut self, centre: math::Vector4, colour: rendering::Colour);
}

impl From<block::BlockType> for rendering::Colour {
    fn from(block_type: block::BlockType) -> Self {
        match block_type {
            block::BlockType::Sea => rendering::Colour::blue(),
            block::BlockType::Sand => rendering::Colour::yellow(),
            block::BlockType::Grass => rendering::Colour::green(),
            block::BlockType::Rock => rendering::Colour::grey(),
            block::BlockType::Ice => rendering::Colour::white(),
            block::BlockType::Air => rendering::Colour::black(),
        }
    }
}

const FACE_SIZE:f32 = 1.0;

#[derive(Debug, Default, Clone)]
pub struct ChunkFrontMesh {
    pub vertices: Vec<mesh::Vertex>
}

#[derive(Debug, Default, Clone)]
pub struct ChunkBackMesh {
    pub vertices: Vec<mesh::Vertex>
}

#[derive(Debug, Default, Clone)]
pub struct ChunkTopMesh {
    pub vertices: Vec<mesh::Vertex>
}

#[derive(Debug, Default, Clone)]
pub struct ChunkRightMesh {
    pub vertices: Vec<mesh::Vertex>
}

#[derive(Debug, Default, Clone)]
pub struct ChunkLeftMesh {
    pub vertices: Vec<mesh::Vertex>
}

impl Tesselate for ChunkFrontMesh {
    fn add_face(&mut self, centre: math::Vector4, colour: rendering::Colour) {
        mesh::CuboidMeshFace::front(centre, FACE_SIZE).tesselate(&mut self.vertices, colour);
    }

    fn get_outermost_blocks<'a>(&mut self, chunk: &'a chunk::Chunk) -> Box<dyn Iterator<Item=block::Block> +'a> {
        chunk.front_most_blocks()
    }
}

impl Tesselate for ChunkBackMesh {
    fn add_face(&mut self, centre: math::Vector4, colour: rendering::Colour) {
        mesh::CuboidMeshFace::back(centre, FACE_SIZE).tesselate(&mut self.vertices, colour);
    }

    fn get_outermost_blocks<'a>(&mut self, chunk: &'a chunk::Chunk) -> Box<dyn Iterator<Item=block::Block> +'a> {
        chunk.back_most_blocks()
    }
}

impl Tesselate for ChunkTopMesh {
    fn add_face(&mut self, centre: math::Vector4, colour: rendering::Colour) {
        mesh::CuboidMeshFace::top(centre, FACE_SIZE).tesselate(&mut self.vertices, colour);
    }

    fn get_outermost_blocks<'a>(&mut self, chunk: &'a chunk::Chunk) -> Box<dyn Iterator<Item=block::Block> +'a> {
        chunk.top_most_blocks()
    }
}
impl Tesselate for ChunkLeftMesh {
    fn add_face(&mut self, centre: math::Vector4, colour: rendering::Colour) {
        mesh::CuboidMeshFace::left(centre, FACE_SIZE).tesselate(&mut self.vertices, colour);
    }

    fn get_outermost_blocks<'a>(&mut self, chunk: &'a chunk::Chunk) -> Box<dyn Iterator<Item=block::Block> +'a> {
        chunk.left_most_blocks()
    }
}

impl Tesselate for ChunkRightMesh {
    fn add_face(&mut self, centre: math::Vector4, colour: rendering::Colour) {
        mesh::CuboidMeshFace::right(centre, FACE_SIZE).tesselate(&mut self.vertices, colour);
    }

    fn get_outermost_blocks<'a>(&mut self, chunk: &'a chunk::Chunk) -> Box<dyn Iterator<Item=block::Block> +'a> {
        chunk.right_most_blocks()
    }
}