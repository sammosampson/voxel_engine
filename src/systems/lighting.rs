use legion::*;
use crate::lighting;
use crate::rendering;
use crate::debug;
use crate::math;

pub fn add_lighting_to_world(world: &mut legion::world::World) {
    world.push((
        lighting::Light {  
            direction: math::Vector4::direction(1.4, 0.4, 0.7)
        },)
    );
}

#[system(for_each)]
pub fn set_render_lighting(
    light: &lighting::Light, 
    #[resource] graph: &mut rendering::WorldRenderGraph
) {
    let timed_block = debug::start_timed_block(debug::CycleCounter::SetRenderLighting);
    graph.light_direction = light.direction;
    timed_block.stop();
}