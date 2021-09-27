use crate::prelude::*;

#[system]
#[read_component(TerrainRevealRadius)]
#[read_component(ChunkPosition)]
#[read_component(Position)]
pub fn reveal_terrain(world: &SubWorld, command_buffer: &mut CommandBuffer, #[resource] graph: &mut WorldRenderGraph,) {
    query_terrain_reveal()
        .iter(world)
        .for_each(|(radius, position)| {
            let positions = get_positions_in_radius(radius, position);
            
            let all_chunks: Vec<(&Entity, &ChunkPosition)> = query_current_chunk_positions()
                .iter(world)
                .collect();

            let all_chunk_positions: Vec<ChunkPosition> = all_chunks
                .iter()
                .map(|(_, position)| **position)
                .collect();

            let chunks_to_add: Vec<ChunkPosition> = positions
                .iter()
                .filter(|position| !all_chunk_positions.contains(*position))
                .map(|position| *position)
                .collect();

            let chunks_to_remove = get_chunks_to_remove(&all_chunks, &positions);

            for position in chunks_to_add {
                println!("adding chunk {:?}", position);
                command_buffer.push((
                    WorldEntityId::from(position),
                    position,
                    full_chunk_shape(),
                    RenderStyle::Fill,
                    Visible(true),
                    ElapsedTime::default(),
                    Rotation::default()
                ));
            }

            for chunk_entity in chunks_to_remove {
                println!("removing chunk {:?}", position);
                graph.remove_node(&chunk_entity);
                command_buffer.remove(chunk_entity);
            }
        })
    
}

fn get_chunks_to_remove(chunks: &Vec<(&Entity, &ChunkPosition)>, where_not_in: &Vec<ChunkPosition>) -> Vec<Entity> {
    chunks
        .iter()
        .filter(|(_, position)| !where_not_in.contains(*position))
        .map(|(entity, _)| **entity)
        .collect()
}

fn query_terrain_reveal<'a>() -> legion::query::Query<(&'a TerrainRevealRadius, &'a Position)> {
    <(&TerrainRevealRadius, &Position)>::query()
}

fn query_current_chunk_positions<'a>() -> legion::query::Query<(Entity, &'a ChunkPosition)> {
    <(Entity, &ChunkPosition)>::query()
}

fn get_positions_in_radius(radius: &TerrainRevealRadius, position: &Position) -> Vec<ChunkPosition> {
    let chunk_position = ChunkPosition::from(*position);
    println!("hero chunk position {:?} {:?}", position, chunk_position);
    vec!(chunk_position)
}