
use crate::prelude::*;

#[system(for_each)]
#[filter(!component::<RenderGraphSet>())]
pub fn build_world_graph_for_mesh(
    entity: &Entity, 
    mesh: &Mesh, 
    #[resource] graph: &mut WorldRenderGraph,
    buffer: &mut CommandBuffer,
) {
    let timed_block = start_timed_block(CycleCounter::BuildWorldGraphForMesh);
    graph.add_mesh(*entity, mesh.data.clone());
    buffer.add_component(*entity, RenderGraphSet::default());
    timed_block.stop();
}