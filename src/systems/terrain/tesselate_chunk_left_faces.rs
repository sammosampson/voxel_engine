use crate::prelude::*;

#[system(for_each)]
#[filter(!component::<ChunkLeftMesh>())]
pub fn tesselate_chunk_left_faces(
    entity: &Entity, 
    chunk: &Chunk, 
    component_buffer: &mut CommandBuffer
) {
    let timed_block = start_timed_block(CycleCounter::TesselateChunkLeftMesh);
    let mut mesh = ChunkLeftMesh::default();
    mesh.tesselate(chunk);
    component_buffer.add_component(*entity, mesh);
    timed_block.stop();
}