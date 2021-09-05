use crate::hero;
use crate::position;
use crate::math;
use crate::mesh;
use crate::rendering;
use crate::cameras;
use crate::input;

pub fn add_hero_to_world(world: &mut legion::world::World) {
    world.push((
        hero::Hero::default(),
        position::Position::from(math::Vector4::position(0.0, 10.0, 0.0)),
        mesh::unit_cube(rendering::Colour::green()),
        cameras::Camera {  
            position: math::Vector4::default(), 
            direction: math::Vector4::direction(0.0, 0.0, 1.0), 
            up: math::Vector4::direction(0.0, 1.0, 0.0) 
        },
        rendering::WindowSize { size: math::Size2d::new(800.0, 600.0) },
        input::MouseInput::default()
    ));
}
