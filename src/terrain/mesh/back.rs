use crate::prelude::*;

#[derive(Debug, Default, Clone)]
pub struct ChunkBackMesh {
    pub vertices: Vec<Vertex>
}

impl Tesselate for ChunkBackMesh {
    fn add_face(&mut self, centre: Vector4, colour: Colour) {
        CuboidMeshFace::back(centre, FACE_SIZE).tesselate(&mut self.vertices, colour);
    }

    fn get_outermost_blocks<'a>(&mut self, chunk: &'a Chunk) -> Box<dyn Iterator<Item=Block> +'a> {
        chunk.back_most_blocks()
    }
}