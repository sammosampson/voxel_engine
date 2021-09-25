use crate::prelude::*;

#[system(for_each)]
#[filter(!component::<ChunkFrontMesh>())]
pub fn tesselate_chunk_front_faces(
    entity: &Entity, 
    chunk: &Chunk, 
    component_buffer: &mut CommandBuffer
) {
    let timed_block = start_timed_block(CycleCounter::TesselateChunkFrontMesh);
    let mut mesh = ChunkFrontMesh::default();
    mesh.tesselate(chunk);
    component_buffer.add_component(*entity, mesh);
    timed_block.stop();
}
