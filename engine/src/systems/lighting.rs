use legion::*;
use voxel_engine_app::lighting;
use voxel_engine_app::rendering;
use voxel_engine_app::debug;
use voxel_engine_app::math;

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
    let timed_block = debug::TimedBlock::start(debug::CycleCounter::SetRenderLighting);
    println!("{:?}", light.direction);
    graph.light_direction = light.direction;
    timed_block.stop();
}