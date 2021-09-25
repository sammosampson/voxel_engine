use crate::prelude::*;

pub fn add_lighting_to_world(world: &mut World) {
    world.push((
        Light {  
            direction: Vector4::direction(1.4, 0.4, 0.7)
        },)
    );
}