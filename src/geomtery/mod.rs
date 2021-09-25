mod vertex;
mod mesh;
mod face;

pub use vertex::*;
pub use mesh::*;
pub use face::*;

use crate::prelude::*;

pub fn unit_cube(colour: Colour) -> Mesh {
    const FACE_SIZE:f32 = 1.0;
    let centre = Vector4::default();
    let mut vertices = vec!();
    CuboidMeshFace::front(centre, FACE_SIZE).tesselate(&mut vertices, colour);
    CuboidMeshFace::back(centre, FACE_SIZE).tesselate(&mut vertices, colour);
    CuboidMeshFace::top(centre, FACE_SIZE).tesselate(&mut vertices, colour);
    CuboidMeshFace::bottom(centre, FACE_SIZE).tesselate(&mut vertices, colour);
    CuboidMeshFace::left(centre, FACE_SIZE).tesselate(&mut vertices, colour);
    CuboidMeshFace::right(centre, FACE_SIZE).tesselate(&mut vertices, colour);
    Mesh::new(vertices)
}