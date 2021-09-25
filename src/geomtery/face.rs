use crate::prelude::*;

pub struct CuboidMeshFace {
    plane: Plane
}

impl CuboidMeshFace {
    pub fn new(plane: Plane) -> Self {
        Self {
            plane 
        }
    }

    pub fn front(centre: Vector4, face_size: f32) -> Self {
        Self::new(Plane::new(
            face_size,
            face_size, 
            centre + Vector4::front() * face_size * 0.5, 
        Vector4::front())
        )
    }
    
    pub fn back(centre: Vector4, face_size: f32) -> Self {
        Self::new(Plane::new(
            face_size,
            face_size, 
            centre + Vector4::back() * face_size * 0.5, 
        Vector4::back())
        )
    }

    pub fn top(centre: Vector4, face_size: f32) -> Self {
        Self::new(Plane::new(
            face_size,
            face_size, 
            centre + Vector4::top() * face_size * 0.5,
        Vector4::top())
        )
    }   

    pub fn bottom(centre: Vector4, face_size: f32) -> Self {
        Self::new(Plane::new(
            face_size,
            face_size, 
            centre + Vector4::bottom() * face_size * 0.5,
        Vector4::bottom())
        )
    }

    pub fn left(centre: Vector4, face_size: f32) -> Self {
        Self::new(Plane::new(
            face_size,
            face_size, 
            centre + Vector4::left() * face_size * 0.5, 
        Vector4::left())
        )    
    }       

    pub fn right(centre: Vector4, face_size: f32) -> Self {
        Self::new(Plane::new(
            face_size,
            face_size, 
            centre + Vector4::right() * face_size * 0.5, 
        Vector4::right())
        )
    }

    pub fn tesselate(&self, vertices: &mut Vec<Vertex>, colour: Colour) {
        CuboidMeshFace::add_vertex(vertices, self.plane.top_right, self.plane.normal, colour, TextureCoordinates::top_right());
        CuboidMeshFace::add_vertex(vertices, self.plane.top_left, self.plane.normal, colour, TextureCoordinates::top_left());
        CuboidMeshFace::add_vertex(vertices, self.plane.bottom_left, self.plane.normal, colour, TextureCoordinates::bottom_left());
        CuboidMeshFace::add_vertex(vertices, self.plane.top_right, self.plane.normal, colour, TextureCoordinates::top_right());
        CuboidMeshFace::add_vertex(vertices, self.plane.bottom_left, self.plane.normal, colour, TextureCoordinates::bottom_left());
        CuboidMeshFace::add_vertex(vertices, self.plane.bottom_right, self.plane.normal, colour, TextureCoordinates::bottom_right());
    }

    fn add_vertex(vertices: &mut Vec<Vertex>, position: Vector4, normal: Vector4, colour: Colour, uv: Vector2) {
        vertices.push(Vertex::new(position, normal, colour, uv));
    }      
}