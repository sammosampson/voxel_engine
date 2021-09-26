use crate::prelude::*;

pub fn add_chunk_to_world(world: &mut World, position: ChunkPosition) {
    world.push((
        WorldEntityId::from(position),
        full_chunk_shape(),
        RenderStyle::Fill,
        position,
        Visible(true),
        ElapsedTime::default(),
        Rotation::default()
        )
    );
}

pub fn add_terrain_to_world(world: &mut World) {
    add_chunk_to_world(world, ChunkPosition::new(0, 0));
    add_chunk_to_world(world, ChunkPosition::new(1, 0));
    add_chunk_to_world(world, ChunkPosition::new(0, 1));
    //add_chunk_to_world(world, ChunkPosition::new(1, 1));
    //add_chunk_to_world(world, ChunkPosition::new(2, 1));
    //add_chunk_to_world(world, ChunkPosition::new(1, 2));
    //add_chunk_to_world(world, ChunkPosition::new(2, 2));
    add_chunk_to_world(world, ChunkPosition::new(0, -1));
    add_chunk_to_world(world, ChunkPosition::new(-1, 0));
}

#[system]
#[read_component(TerrainRevealRadius)]
#[read_component(Position)]
pub fn reveal_terrain(world: &mut SubWorld, command_buffer: &mut CommandBuffer) {
    query_terrain_reveal()
        .iter(world)
        .for_each(|(radius, position)| {
            let positions = get_positions_in_radius(radius, position);
            
            let current_chunk_positions = query_current_chunk_positions()
                .iter(world);

            let chunks_to_remove = current_chunk_positions
                .filter(|(entity, _)|)
                

            // add new chunks not in radius
        })
    
}

fn query_terrain_reveal<'a>() -> legion::query::Query<(&'a TerrainRevealRadius, &'a Position)> {
    <(&TerrainRevealRadius, &Position)>::query()
}

fn query_current_chunk_positions<'a>() -> legion::query::Query<(Entity, &'a ChunkPosition)> {
    <(Entity, &ChunkPosition)>::query()
}

fn get_positions_in_radius(radius: &TerrainRevealRadius, postion: &Position) -> Vec<ChunkPosition> {
    // convert position to ChunkPosition
    vec!()
}