use crate::prelude::*;

#[system(for_each)]
#[filter(component::<RenderGraphSet>())]
pub fn set_world_node_position(
    entity: &Entity,
    position: &Position, 
    #[resource] render_graph: &mut WorldRenderGraph
    ) {
        let timed_block = start_timed_block(CycleCounter::SetWorldNodeOrientation);
        
        render_graph
            .find(entity)
            .unwrap()
            .set_position(Matrix4x4::translation(position.value));
        
        timed_block.stop();
}
