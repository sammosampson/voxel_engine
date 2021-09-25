use crate::prelude::*;

#[system(for_each)]
#[filter(component::<ChunkPosition>())]
#[filter(!component::<Position>())]
pub fn position_chunks(
    entity: &Entity, 
    chunk_position: &ChunkPosition, 
    component_buffer: &mut CommandBuffer
) {
    let timed_block = start_timed_block(CycleCounter::PositionChunks);
    let world_position = Position::from(chunk_position.absolute_centre());
    component_buffer.add_component(*entity, world_position);
    timed_block.stop();
}