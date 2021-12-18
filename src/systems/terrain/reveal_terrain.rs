use crate::prelude::*;

#[system(for_each)]
#[filter(component::<InitialTerrainBuilt>())]
#[write_component(ChunkPosition)]
pub fn reveal_terrain(world: &mut SubWorld, position: &Position, last_position: &LastPosition) {
    let timed_block = start_timed_block(CycleCounter::RevealTerrain);

    let reveal_area_chunk_position = ChunkPosition::from(**last_position);
    let new_reveal_area_chunk_position = ChunkPosition::from(**position);
    
    if reveal_area_chunk_position == new_reveal_area_chunk_position {
        return;
    }

    let chunk_offset = new_reveal_area_chunk_position - reveal_area_chunk_position;

    println!("{:?} {:?} {:?}",  reveal_area_chunk_position, new_reveal_area_chunk_position, chunk_offset);

    <&mut ChunkPosition>::query()
        .iter_mut(world)
        .for_each(|chunk_position| *chunk_position = *chunk_position + chunk_offset);        

    timed_block.stop();
}