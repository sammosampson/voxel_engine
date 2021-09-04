use crate::mesh;
use crate::rendering;
use crate::math;
use super::chunk;
use super::block;

const FACE_SIZE:f32 = 1.0;
pub struct ChunkMeshFace {
    plane: math::Plane
}

impl ChunkMeshFace {
    fn new(plane: math::Plane) -> Self {
        Self {
            plane 
        }
    }

    fn front(centre: math::Vector4) -> Self {
        Self::new(math::Plane::new(
            FACE_SIZE,
            FACE_SIZE, 
            centre + math::Vector4::position(0.0, 0.0, -0.5), 
        math::Vector4::front())
        )
    }
    
    fn back(centre: math::Vector4) -> Self {
        Self::new(math::Plane::new(
            FACE_SIZE,
            FACE_SIZE, 
            centre + math::Vector4::position(0.0, 0.0, 0.5), 
        math::Vector4::back())
        )
    }

    fn top(centre: math::Vector4) -> Self {
        Self::new(math::Plane::new(
            FACE_SIZE,
            FACE_SIZE, 
            centre + math::Vector4::position(0.0, 0.5, 0.0), 
        math::Vector4::top())
        )
    }   

    fn bottom(centre: math::Vector4) -> Self {
        Self::new(math::Plane::new(
            FACE_SIZE,
            FACE_SIZE, 
            centre + math::Vector4::position(0.0, -0.5,0.0), 
        math::Vector4::bottom())
        )
    }

    fn left(centre: math::Vector4) -> Self {
        Self::new(math::Plane::new(
            FACE_SIZE,
            FACE_SIZE, 
            centre + math::Vector4::position(-0.5, 0.0, 0.0), 
        math::Vector4::left())
        )    
    }       

    fn right(centre: math::Vector4) -> Self {
        Self::new(math::Plane::new(
            FACE_SIZE,
            FACE_SIZE, 
            centre + math::Vector4::position(0.5, 0.0, 0.0), 
        math::Vector4::right())
        )
    }

    fn tesselate(&self, vertices: &mut Vec<mesh::Vertex>, colour: rendering::Colour) {
        ChunkMeshFace::add_vertex(vertices, self.plane.top_right, self.plane.normal, colour, math::Vector2::new(1.0, 1.0));
        ChunkMeshFace::add_vertex(vertices, self.plane.top_left, self.plane.normal, colour, math::Vector2::new(0.0, 1.0));
        ChunkMeshFace::add_vertex(vertices, self.plane.bottom_left, self.plane.normal, colour, math::Vector2::new(0.0, 0.0));
        ChunkMeshFace::add_vertex(vertices, self.plane.top_right, self.plane.normal, colour, math::Vector2::new(1.0, 1.0));
        ChunkMeshFace::add_vertex(vertices, self.plane.bottom_left, self.plane.normal, colour, math::Vector2::new(0.0, 0.0));
        ChunkMeshFace::add_vertex(vertices, self.plane.bottom_right, self.plane.normal, colour, math::Vector2::new(1.0, 0.0));
    }

    fn add_vertex(vertices: &mut Vec<mesh::Vertex>, position: math::Vector4, normal: math::Vector4, colour: rendering::Colour, uv: math::Vector2) {
        vertices.push(mesh::Vertex::new(position, normal, colour, uv));
    }      
}

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

#[derive(Debug, Default, Clone)]
pub struct ChunkLeftMesh {
    pub vertices: Vec<mesh::Vertex>
}

impl Tesselate for ChunkFrontMesh {
    fn add_face(&mut self, centre: math::Vector4, colour: rendering::Colour) {
        ChunkMeshFace::front(centre).tesselate(&mut self.vertices, colour);
    }
}

impl Tesselate for ChunkBackMesh {
    fn add_face(&mut self, centre: math::Vector4, colour: rendering::Colour) {
        ChunkMeshFace::back(centre).tesselate(&mut self.vertices, colour);
    }
}

impl Tesselate for ChunkTopMesh {
    fn add_face(&mut self, centre: math::Vector4, colour: rendering::Colour) {
        ChunkMeshFace::top(centre).tesselate(&mut self.vertices, colour);
    }
}

impl Tesselate for ChunkBottomMesh {
    fn add_face(&mut self, centre: math::Vector4, colour: rendering::Colour) {
        ChunkMeshFace::bottom(centre).tesselate(&mut self.vertices, colour);
    }
}

impl Tesselate for ChunkLeftMesh {
    fn add_face(&mut self, centre: math::Vector4, colour: rendering::Colour) {
        ChunkMeshFace::left(centre).tesselate(&mut self.vertices, colour);
    }
}

impl Tesselate for ChunkRightMesh {
    fn add_face(&mut self, centre: math::Vector4, colour: rendering::Colour) {
        ChunkMeshFace::right(centre).tesselate(&mut self.vertices, colour);
    }
}