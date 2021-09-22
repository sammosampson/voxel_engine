use legion::*;
use legion::systems::CommandBuffer;
use crate::debug;
use crate::world;
use crate::rendering;
use crate::position;
use crate::terrain;
use crate::terrain::*;

pub fn add_terrain_to_world(world: &mut legion::world::World) {
    world.push((
        world::WorldEntityId::from("chunk+0+0"),
        full_chunk_shape(),
        terrain::ChunkPosition::new(0, 0),
        world::Visible(true)
        )
    );

    world.push((
        world::WorldEntityId::from("chunk+1+0"),
        full_chunk_shape(),
        terrain::ChunkPosition::new(1, 0),
        world::Visible(true),
        )
    );

    world.push((
        world::WorldEntityId::from("chunk-1+0"),
        full_chunk_shape(),
        terrain::ChunkPosition::new(-1, 0),
        world::Visible(true),
        )
    );

    world.push((
        world::WorldEntityId::from("chunk+0+1"),
        full_chunk_shape(),
        terrain::ChunkPosition::new(0, 1),
        world::Visible(true),
        )
    );

    world.push((
        world::WorldEntityId::from("chunk+1-1"),
        full_chunk_shape(),
        terrain::ChunkPosition::new(0, -1),
        world::Visible(true),
        )
    );
}

#[system(for_each)]
#[filter(component::<terrain::ChunkPosition>())]
#[filter(!component::<position::Position>())]
pub fn position_chunks(
    entity: &Entity, 
    chunk_position: &terrain::ChunkPosition, 
    component_buffer: &mut CommandBuffer
) {
    let timed_block = debug::start_timed_block(debug::CycleCounter::PositionChunks);
    let world_position = position::Position::from(chunk_position.absolute_centre());
    component_buffer.add_component(*entity, world_position);
    timed_block.stop();
}

#[system(for_each)]
#[filter(!component::<terrain::ChunkFrontMesh>())]
pub fn tesselate_chunk_front_faces(
    entity: &Entity, 
    chunk: &terrain::Chunk, 
    component_buffer: &mut CommandBuffer
) {
    let timed_block = debug::start_timed_block(debug::CycleCounter::TesselateChunkFrontMesh);
    let mut mesh = terrain::ChunkFrontMesh::default();
    mesh.tesselate(chunk);
    component_buffer.add_component(*entity, mesh);
    timed_block.stop();
}

#[system(for_each)]
#[filter(!component::<terrain::ChunkBackMesh>())]
pub fn tesselate_chunk_back_faces(
    entity: &Entity, 
    chunk: &terrain::Chunk, 
    component_buffer: &mut CommandBuffer
) {
    let timed_block = debug::start_timed_block(debug::CycleCounter::TesselateChunkBackMesh);
    let mut mesh = terrain::ChunkBackMesh::default();
    mesh.tesselate(chunk);
    component_buffer.add_component(*entity, mesh);
    timed_block.stop();
}

#[system(for_each)]
#[filter(!component::<terrain::ChunkTopMesh>())]
pub fn tesselate_chunk_top_faces(
    entity: &Entity, 
    chunk: &terrain::Chunk, 
    component_buffer: &mut CommandBuffer
) {
    let timed_block = debug::start_timed_block(debug::CycleCounter::TesselateChunkTopMesh);
    let mut mesh = terrain::ChunkTopMesh::default();
    mesh.tesselate(chunk);
    component_buffer.add_component(*entity, mesh);
    timed_block.stop();
}

#[system(for_each)]
#[filter(!component::<terrain::ChunkLeftMesh>())]
pub fn tesselate_chunk_left_faces(
    entity: &Entity, 
    chunk: &terrain::Chunk, 
    component_buffer: &mut CommandBuffer
) {
    let timed_block = debug::start_timed_block(debug::CycleCounter::TesselateChunkLeftMesh);
    let mut mesh = terrain::ChunkLeftMesh::default();
    mesh.tesselate(chunk);
    component_buffer.add_component(*entity, mesh);
    timed_block.stop();
}

#[system(for_each)]
#[filter(!component::<terrain::ChunkRightMesh>())]
pub fn tesselate_chunk_right_faces(
    entity: &Entity, 
    chunk: &terrain::Chunk, 
    component_buffer: &mut CommandBuffer
) {
    let timed_block = debug::start_timed_block(debug::CycleCounter::TesselateChunkRightMesh);
    let mut mesh = terrain::ChunkRightMesh::default();
    mesh.tesselate(chunk);
    component_buffer.add_component(*entity, mesh);
    timed_block.stop();
}

#[system(for_each)]
#[filter(!component::<rendering::RenderGraphSet>())]
pub fn merge_chunk_mesh(
    entity: &Entity, 
    front: &terrain::ChunkFrontMesh, 
    back: &terrain::ChunkBackMesh, 
    top: &terrain::ChunkTopMesh, 
    left: &terrain::ChunkLeftMesh, 
    right: &terrain::ChunkRightMesh,
    #[resource] graph: &mut rendering::WorldRenderGraph,
    buffer: &mut CommandBuffer,
) {
    let timed_block = debug::start_timed_block(debug::CycleCounter::MergeChunkMesh);
    let mut vertices = vec!();
    vertices.append(&mut front.clone().vertices);
    vertices.append(&mut back.clone().vertices);
    vertices.append(&mut top.clone().vertices);
    vertices.append(&mut left.clone().vertices);
    vertices.append(&mut right.clone().vertices);
    graph.add_mesh(*entity, vertices);
    buffer.add_component(*entity, rendering::RenderGraphSet::default());
    timed_block.stop();
}
