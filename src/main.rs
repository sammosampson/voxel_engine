mod application;
mod systems;
mod terrain;
mod graph;
mod rendering;
mod events;
mod debug;
mod math;
mod time;
mod geomtery;
mod input;
mod cameras;
mod lighting;
mod position;
mod rotation;
mod hero;
mod physics;
mod world;
mod game_state;

mod prelude {
    pub use std::collections::HashMap; 
    pub use std::time::Instant; 
    pub use itertools::Itertools; 
    pub use shrev::{EventChannel, EventIterator};
    pub use legion::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use crate::game_state::*;
    pub use crate::world::*;
    pub use crate::rendering::*;
    pub use crate::events::*;
    pub use crate::debug::*;
    pub use crate::cameras::*;
    pub use crate::math::*;
    pub use crate::input::*;
    pub use crate::position::*;
    pub use crate::physics::*;
    pub use crate::time::*;
    pub use crate::hero::*;
    pub use crate::geomtery::*;
    pub use crate::rotation::*;
    pub use crate::lighting::*;
    pub use crate::graph::*;
    pub use crate::terrain::*;
    pub use crate::systems::*;
}

fn main() {
    application::Application::build().run();
}
