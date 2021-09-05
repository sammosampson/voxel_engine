use crate::mesh;
use crate::rendering;
use crate::math;
use super::chunk;
use super::block;


pub trait Tesselate {
    fn tesselate(&mut self, chunk: &chunk::Chunk) {
        for block in chunk::ChunkIterator::new(chunk) {
            match block.block_type {
                block::BlockType::Air => {},
                block::BlockType::Grass => self.add_face( block.absolute_position(), rendering::Colour::green()),
                block::BlockType::Brick => self.add_face( block.absolute_position() ,rendering::Colour::grey()),
            }
        }
    }

    fn add_face(&mut self, centre: math::Vector4, colour: rendering::Colour);
}


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
pub struct ChunkBottomMesh {
    pub vertices: Vec<mesh::Vertex>
}

#[derive(Debug, Default, Clone)]
pub struct ChunkRightMesh {
    pub vertices: Vec<mesh::Vertex>
}

const FACE_SIZE:f32 = 1.0;

#[derive(Debug, Default, Clone)]
pub struct ChunkLeftMesh {
    pub vertices: Vec<mesh::Vertex>
}

impl Tesselate for ChunkFrontMesh {
    fn add_face(&mut self, centre: math::Vector4, colour: rendering::Colour) {
        mesh::CuboidMeshFace::front(centre, FACE_SIZE).tesselate(&mut self.vertices, colour);
    }
}

impl Tesselate for ChunkBackMesh {
    fn add_face(&mut self, centre: math::Vector4, colour: rendering::Colour) {
        mesh::CuboidMeshFace::back(centre, FACE_SIZE).tesselate(&mut self.vertices, colour);
    }
}

impl Tesselate for ChunkTopMesh {
    fn add_face(&mut self, centre: math::Vector4, colour: rendering::Colour) {
        mesh::CuboidMeshFace::top(centre, FACE_SIZE).tesselate(&mut self.vertices, colour);
    }
}

impl Tesselate for ChunkBottomMesh {
    fn add_face(&mut self, centre: math::Vector4, colour: rendering::Colour) {
        mesh::CuboidMeshFace::bottom(centre, FACE_SIZE).tesselate(&mut self.vertices, colour);
    }
}

impl Tesselate for ChunkLeftMesh {
    fn add_face(&mut self, centre: math::Vector4, colour: rendering::Colour) {
        mesh::CuboidMeshFace::left(centre, FACE_SIZE).tesselate(&mut self.vertices, colour);
    }
}

impl Tesselate for ChunkRightMesh {
    fn add_face(&mut self, centre: math::Vector4, colour: rendering::Colour) {
        mesh::CuboidMeshFace::right(centre, FACE_SIZE).tesselate(&mut self.vertices, colour);
    }
}