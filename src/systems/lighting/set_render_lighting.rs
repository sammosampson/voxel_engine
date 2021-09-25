use crate::prelude::*;

#[system(for_each)]
pub fn set_render_lighting(
    light: &Light, 
    #[resource] graph: &mut WorldRenderGraph
) {
    let timed_block = start_timed_block(CycleCounter::SetRenderLighting);
    graph.light_direction = light.direction;
    timed_block.stop();
}