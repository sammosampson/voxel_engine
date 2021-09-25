use crate::prelude::*;

#[system(for_each)]
pub fn set_world_visibility(
    entity: &Entity,
    visible: &Visible, 
    #[resource] graph: &mut WorldRenderGraph
) {
    graph.find(entity).unwrap().set_visibility(visible.0);
}