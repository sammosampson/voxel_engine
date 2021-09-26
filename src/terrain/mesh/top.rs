use crate::prelude::*;

#[derive(Debug, Default, Clone)]
pub struct ChunkTopMesh {
    pub vertices: Vec<Vertex>
}

impl Tesselate for ChunkTopMesh {
    fn add_face(&mut self, centre: Vector4, colour: Colour) {
        CuboidMeshFace::top(centre, FACE_SIZE).tesselate(&mut self.vertices, colour);
    }

    fn get_outermost_blocks<'a>(&mut self, chunk: &'a Chunk) -> Box<dyn Iterator<Item=Block> +'a> {
        chunk.top_most_blocks()
    }
}
