use crate::prelude::*;

pub fn add_hero_to_world(world: &mut World) {
    world.push((HeroSpawner, Position::from(Vector4::position(0.0, 10.0, 0.0))));
}