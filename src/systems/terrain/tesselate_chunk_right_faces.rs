use crate::prelude::*;

#[system(for_each)]
#[filter(!component::<ChunkRightMesh>())]
pub fn tesselate_chunk_right_faces(
    entity: &Entity, 
    chunk: &Chunk, 
    component_buffer: &mut CommandBuffer
) {
    let timed_block = start_timed_block(CycleCounter::TesselateChunkRightMesh);
    let mut mesh = ChunkRightMesh::default();
    mesh.tesselate(chunk);
    component_buffer.add_component(*entity, mesh);
    timed_block.stop();
}