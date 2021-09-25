use crate::prelude::*;

#[system(for_each)]
#[filter(!component::<RenderGraphSet>())]
pub fn merge_chunk_mesh(
    entity: &Entity, 
    front: &ChunkFrontMesh, 
    back: &ChunkBackMesh, 
    top: &ChunkTopMesh, 
    left: &ChunkLeftMesh, 
    right: &ChunkRightMesh,
    #[resource] graph: &mut WorldRenderGraph,
    buffer: &mut CommandBuffer,
) {
    let timed_block = start_timed_block(CycleCounter::MergeChunkMesh);
    let mut vertices = vec!();
    vertices.append(&mut front.clone().vertices);
    vertices.append(&mut back.clone().vertices);
    vertices.append(&mut top.clone().vertices);
    vertices.append(&mut left.clone().vertices);
    vertices.append(&mut right.clone().vertices);
    graph.add_mesh(*entity, vertices);
    buffer.add_component(*entity, RenderGraphSet::default());
    timed_block.stop();
}
