use crate::prelude::*;

pub fn add_terrain_to_world(world: &mut legion::World) {
    world.push((
        WorldEntityId::from("chunk+0+0"),
        full_chunk_shape(),
        RenderStyle::Fill,
        ChunkPosition::new(0, 0),
        Visible(true),
        ElapsedTime::default(),
        Rotation::default()
        )
    );

    world.push((
        WorldEntityId::from("chunk+1+0"),
        full_chunk_shape(),
        RenderStyle::Fill,
        ChunkPosition::new(1, 0),
        Visible(true),
        ElapsedTime::default(),
        Rotation::default()
        )
    );

    world.push((
        WorldEntityId::from("chunk-1+0"),
        full_chunk_shape(),
        RenderStyle::Fill,
        ChunkPosition::new(-1, 0),
        Visible(true),
        ElapsedTime::default(),
        Rotation::default()
        )
    );

    world.push((
        WorldEntityId::from("chunk+0+1"),
        full_chunk_shape(),
        RenderStyle::Fill,
        ChunkPosition::new(0, 1),
        Visible(true),
        ElapsedTime::default(),
        Rotation::default()
        )
    );

    world.push((
        WorldEntityId::from("chunk+1-1"),
        full_chunk_shape(),
        RenderStyle::Fill,
        ChunkPosition::new(0, -1),
        Visible(true),
        ElapsedTime::default(),
        Rotation::default()
        )
    );
}