use crate::prelude::*;

#[derive(Debug, Default, Clone)]
pub struct ChunkFrontMesh {
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
