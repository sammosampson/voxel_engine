use voxel_engine_app::mesh;
use voxel_engine_app::rendering;
use voxel_engine_app::math;
use voxel_engine_app::terrain;
use super::chunk;

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
    fn tesselate(&mut self, chunk: &terrain::Chunk) {
        for block in chunk::ChunkIterator::new(chunk) {
            match block.block_type {
                terrain::BlockType::Air => {},
                terrain::BlockType::Grass => self.add_face( block.absolute_position(), rendering::Colour::green()),
                terrain::BlockType::Brick => self.add_face( block.absolute_position() ,rendering::Colour::grey()),
            }
        }
    }

    fn add_face(&mut self, centre: math::Vector4, colour: rendering::Colour);
}

impl Tesselate for terrain::ChunkFrontMesh {
    fn add_face(&mut self, centre: math::Vector4, colour: rendering::Colour) {
        ChunkMeshFace::front(centre).tesselate(&mut self.vertices, colour);
    }
}

impl Tesselate for terrain::ChunkBackMesh {
    fn add_face(&mut self, centre: math::Vector4, colour: rendering::Colour) {
        ChunkMeshFace::back(centre).tesselate(&mut self.vertices, colour);
    }
}

impl Tesselate for terrain::ChunkTopMesh {
    fn add_face(&mut self, centre: math::Vector4, colour: rendering::Colour) {
        ChunkMeshFace::top(centre).tesselate(&mut self.vertices, colour);
    }
}

impl Tesselate for terrain::ChunkBottomMesh {
    fn add_face(&mut self, centre: math::Vector4, colour: rendering::Colour) {
        ChunkMeshFace::bottom(centre).tesselate(&mut self.vertices, colour);
    }
}

impl Tesselate for terrain::ChunkLeftMesh {
    fn add_face(&mut self, centre: math::Vector4, colour: rendering::Colour) {
        ChunkMeshFace::left(centre).tesselate(&mut self.vertices, colour);
    }
}

impl Tesselate for terrain::ChunkRightMesh {
    fn add_face(&mut self, centre: math::Vector4, colour: rendering::Colour) {
        ChunkMeshFace::right(centre).tesselate(&mut self.vertices, colour);
    }
}