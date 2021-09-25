use crate::prelude::*;

#[system(for_each)]
#[filter(component::<RenderGraphSet>())]
pub fn set_world_node_rotation(
    entity: &Entity,
    rotation: &Rotation, 
    #[resource] render_graph: &mut WorldRenderGraph
    ) {
        let timed_block = start_timed_block(CycleCounter::SetWorldNodeOrientation);
        
        render_graph
            .find(entity)
            .unwrap()
            .set_rotation(Matrix4x4::x_rotation(rotation.x) * Matrix4x4::y_rotation(rotation.y));
        
        timed_block.stop();
}