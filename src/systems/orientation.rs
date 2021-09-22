
use legion::*;
use crate::debug;
use crate::math;
use crate::position;
use crate::rotation;
use crate::rendering;
use crate::time;

#[system(par_for_each)]
pub fn constant_rotation(
    time: &time::ElapsedTime,
    speed: &rotation::ConstantRotation,
    rotation: &mut rotation::Rotation
) {   
    let timed_block = debug::start_timed_block(debug::CycleCounter::ConstantRotation);
    rotation.x = calculate_rotation(rotation.x, speed.x_revoloutions_per_second, time);
    rotation.y = calculate_rotation(rotation.y, speed.y_revoloutions_per_second, time);
    timed_block.stop();
}

fn calculate_rotation(current_rotation: f32, speed:f32, time: &time::ElapsedTime) -> f32 {
    let two_pi = math::PI * 2.0;
    let elapsed_revolutions = time.seconds as f32 * speed;
    let total_radians = (elapsed_revolutions * two_pi) + current_rotation;
    total_radians % two_pi
}

#[system(for_each)]
#[filter(component::<rendering::RenderGraphSet>())]
pub fn set_world_node_rotation(
    entity: &Entity,
    rotation: &rotation::Rotation, 
    #[resource] render_graph: &mut rendering::WorldRenderGraph
    ) {
        let timed_block = debug::start_timed_block(debug::CycleCounter::SetWorldNodeOrientation);
        
        render_graph
            .find(entity)
            .unwrap()
            .set_rotation(math::Matrix4x4::x_rotation(rotation.x) * math::Matrix4x4::y_rotation(rotation.y));
        
        timed_block.stop();
}

#[system(for_each)]
#[filter(component::<rendering::RenderGraphSet>())]
pub fn set_world_node_position(
    entity: &Entity,
    position: &position::Position, 
    #[resource] render_graph: &mut rendering::WorldRenderGraph
    ) {
        let timed_block = debug::start_timed_block(debug::CycleCounter::SetWorldNodeOrientation);
        
        render_graph
            .find(entity)
            .unwrap()
            .set_position(math::Matrix4x4::translation(position.value));
        
        timed_block.stop();
}
