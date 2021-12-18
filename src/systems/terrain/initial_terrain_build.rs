use crate::prelude::*;

#[system(for_each)]
#[filter(!component::<InitialTerrainBuilt>())]
pub fn initial_terrain_build(
    entity: &Entity,
    position: &Position,
    terrain_area_size: &TerrainRevealAreaSize,
    command_buffer: &mut CommandBuffer
) {
    let terrain_reveal_chunk = ChunkPosition::from(**position);
    let reveal_area_size: i64 = (terrain_area_size.0).into();
    let half_reveal_area_size = (reveal_area_size - 1) / 2;
    let top_corner_offset_chunk = ChunkPosition::new(-half_reveal_area_size, -half_reveal_area_size);
    let top_corner_chunk = terrain_reveal_chunk + top_corner_offset_chunk;
    let chunk_shape = full_chunk_shape();

    for x in 0..reveal_area_size {
        for z in 0..reveal_area_size {        
            command_buffer.push((top_corner_chunk + ChunkPosition::new(x, z), chunk_shape.clone()));
        }
    }

    command_buffer.add_component(*entity, InitialTerrainBuilt);
}