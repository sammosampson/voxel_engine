use super::vectors;

pub struct Plane {
    pub top_left: vectors::Vector4, 
    pub top_right: vectors::Vector4, 
    pub bottom_left: vectors::Vector4, 
    pub bottom_right: vectors::Vector4, 
    pub normal: vectors::Vector4
}

impl Plane {
    pub fn new(
        width: f32, 
        height: f32, 
        centre: vectors::Vector4, 
        normal: vectors::Vector4) -> Self {

        let (u, w) = Plane::get_uw(normal);
        let u_sized =  u * (width / 2.0);
        let w_sized = w * (height / 2.0);

        Self {
            top_left: centre + u_sized + w_sized, 
            top_right: centre - u_sized + w_sized, 
            bottom_left: centre + u_sized - w_sized, 
            bottom_right: centre - u_sized - w_sized, 
            normal
        }
    }

    fn get_uw(normal: vectors::Vector4) -> (vectors::Vector4, vectors::Vector4) {
        let up = vectors::Vector4::up();
        if normal == up {
            return (vectors::Vector4::left(), vectors::Vector4::back())
        }
        if normal == vectors::Vector4::down() {
            return (vectors::Vector4::left(), vectors::Vector4::front())
        }

        let u = vectors::Vector4::up().cross(normal);
        let w = normal.cross(u);
        (u, w)
    }
}

#[test]
fn new_plane_produces_correct_coordinates() {
    let normal = vectors::Vector4::direction(0.0, 0.0, -1.0);
    let centre = vectors::Vector4::position(0.0, 0.0, -0.5);

    let plane = Plane::new(1.0, 1.0, centre, normal);
    assert_eq!(plane.top_left, vectors::Vector4::position(-0.5, 0.5, -0.5)); 
    assert_eq!(plane.top_right, vectors::Vector4::position(0.5,  0.5, -0.5)); 
    assert_eq!(plane.bottom_left, vectors::Vector4::position(-0.5, -0.5, -0.5));
    assert_eq!(plane.bottom_right, vectors::Vector4::position(0.5, -0.5, -0.5));
    assert_eq!(plane.normal, normal);
}

#[test]
fn new_top_facing_plane_produces_correct_coordinates() {
    let normal = vectors::Vector4::direction(0.0, 1.0, 0.0);
    let centre = vectors::Vector4::position(0.0, 0.5, 0.0);
    
    let plane = Plane::new(1.0, 1.0, centre, normal);
    assert_eq!(plane.top_left, vectors::Vector4::position(0.5, 0.5, 0.5)); 
    assert_eq!(plane.top_right, vectors::Vector4::position(-0.5, 0.5, 0.5)); 
    assert_eq!(plane.bottom_left, vectors::Vector4::position(0.5, 0.5, -0.5));
    assert_eq!(plane.bottom_right, vectors::Vector4::position(-0.5, 0.5, -0.5));
    assert_eq!(plane.normal, normal);
}

#[test]
fn new_bottom_facing_plane_produces_correct_coordinates() {
    let normal = vectors::Vector4::direction(0.0, -1.0, 0.0);
    let centre = vectors::Vector4::position(0.0, -0.5, 0.0);
    
    let plane = Plane::new(1.0, 1.0, centre, normal);
    assert_eq!(plane.top_left, vectors::Vector4::position(-0.5, -0.5, -0.5)); 
    assert_eq!(plane.top_right, vectors::Vector4::position(0.5, -0.5, -0.5)); 
    assert_eq!(plane.bottom_left, vectors::Vector4::position(-0.5, -0.5, 0.5));
    assert_eq!(plane.bottom_right, vectors::Vector4::position(0.5, -0.5, 0.5));
    assert_eq!(plane.normal, normal);
}

