use crate::math;
use crate::rendering;


#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: math::Vector4,
    pub normal: math::Vector4,
    pub colour: rendering::Colour,
    pub uv: math::Vector2
}

impl Vertex {
    pub fn new(position: math::Vector4, normal: math::Vector4, colour: rendering::Colour, uv: math::Vector2) -> Self {
        Self {
            position,
            normal,
            colour,
            uv
        }
    }
}

#[derive(Debug)]
pub struct Mesh {
    pub data: Vec<Vertex>,
}


impl Mesh {
    pub fn new(data: Vec<Vertex>) -> Self {
        Self {
            data
        }
    }
}

pub fn unit_cube(colour: rendering::Colour) -> Mesh {
    const FACE_SIZE:f32 = 1.0;
    let centre = math::Vector4::default();
    let mut vertices = vec!();
    CuboidMeshFace::front(centre, FACE_SIZE).tesselate(&mut vertices, colour);
    CuboidMeshFace::back(centre, FACE_SIZE).tesselate(&mut vertices, colour);
    CuboidMeshFace::top(centre, FACE_SIZE).tesselate(&mut vertices, colour);
    CuboidMeshFace::bottom(centre, FACE_SIZE).tesselate(&mut vertices, colour);
    CuboidMeshFace::left(centre, FACE_SIZE).tesselate(&mut vertices, colour);
    CuboidMeshFace::right(centre, FACE_SIZE).tesselate(&mut vertices, colour);
    Mesh::new(vertices)
}

pub struct CuboidMeshFace {
    plane: math::Plane
}

impl CuboidMeshFace {
    pub fn new(plane: math::Plane) -> Self {
        Self {
            plane 
        }
    }

    pub fn front(centre: math::Vector4, face_size: f32) -> Self {
        Self::new(math::Plane::new(
            face_size,
            face_size, 
            centre +math::Vector4::front() * face_size * 0.5, 
        math::Vector4::front())
        )
    }
    
    pub fn back(centre: math::Vector4, face_size: f32) -> Self {
        Self::new(math::Plane::new(
            face_size,
            face_size, 
            centre + math::Vector4::back() * face_size * 0.5, 
        math::Vector4::back())
        )
    }

    pub fn top(centre: math::Vector4, face_size: f32) -> Self {
        Self::new(math::Plane::new(
            face_size,
            face_size, 
            centre + math::Vector4::top() * face_size * 0.5,
        math::Vector4::top())
        )
    }   

    pub fn bottom(centre: math::Vector4, face_size: f32) -> Self {
        Self::new(math::Plane::new(
            face_size,
            face_size, 
            centre + math::Vector4::bottom() * face_size * 0.5,
        math::Vector4::bottom())
        )
    }

    pub fn left(centre: math::Vector4, face_size: f32) -> Self {
        Self::new(math::Plane::new(
            face_size,
            face_size, 
            centre + math::Vector4::left() * face_size * 0.5, 
        math::Vector4::left())
        )    
    }       

    pub fn right(centre: math::Vector4, face_size: f32) -> Self {
        Self::new(math::Plane::new(
            face_size,
            face_size, 
            centre + math::Vector4::right() * face_size * 0.5, 
        math::Vector4::right())
        )
    }

    pub fn tesselate(&self, vertices: &mut Vec<Vertex>, colour: rendering::Colour) {
        CuboidMeshFace::add_vertex(vertices, self.plane.top_right, self.plane.normal, colour, math::TextureCoordinates::top_right());
        CuboidMeshFace::add_vertex(vertices, self.plane.top_left, self.plane.normal, colour, math::TextureCoordinates::top_left());
        CuboidMeshFace::add_vertex(vertices, self.plane.bottom_left, self.plane.normal, colour, math::TextureCoordinates::bottom_left());
        CuboidMeshFace::add_vertex(vertices, self.plane.top_right, self.plane.normal, colour, math::TextureCoordinates::top_right());
        CuboidMeshFace::add_vertex(vertices, self.plane.bottom_left, self.plane.normal, colour, math::TextureCoordinates::bottom_left());
        CuboidMeshFace::add_vertex(vertices, self.plane.bottom_right, self.plane.normal, colour, math::TextureCoordinates::bottom_right());
    }

    fn add_vertex(vertices: &mut Vec<Vertex>, position: math::Vector4, normal: math::Vector4, colour: rendering::Colour, uv: math::Vector2) {
        vertices.push(Vertex::new(position, normal, colour, uv));
    }      
}