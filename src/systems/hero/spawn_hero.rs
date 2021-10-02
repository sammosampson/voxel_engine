use crate::prelude::*;

#[system(for_each)]
#[filter(component::<HeroSpawner>())]
#[filter(!component::<HeroSpawned>())]
pub fn spawn_hero(
    entity: &Entity,
    position: &Position,
    command_buffer: &mut CommandBuffer
) {
    command_buffer.add_component(*entity, HeroSpawned);

    let world_id = WorldEntityId::from("hero");
    let rotation = Rotation::default();
    let velocity = Velocity::default();
    let heading = Heading::from(Vector4::direction(0.0, 0.0, 1.0));
    let time = ElapsedTime::default();
    let mesh =  unit_cube(Colour::green());
    let input = MouseInput::default();
    
    let attach_camera = AttachCamera {  
        offset_position: Vector4::position(0.0, 1.0, 0.0), 
        offset_direction: Vector4::direction(0.0, -0.2, 0.0)
    };

    let camera = Camera {  
        position:  Vector4::default(), 
        direction: Vector4::default(), 
        up: Vector4::up()
    };

    let window_size = WindowSize {
        size: Size2d::new(800.0, 600.0)
    };

    let visible = Visible(true);
    let render_style = RenderStyle::Fill;
    let terrain_revealer = TerrainRevealAreaSize(2);
    
    let spawned = command_buffer.push((
        world_id,
        *position,
        rotation, 
        velocity,
        heading,
        time,
        mesh,
        input, 
    ));

    command_buffer.add_component(spawned, attach_camera);
    command_buffer.add_component(spawned, camera);
    command_buffer.add_component(spawned, window_size);
    command_buffer.add_component(spawned, visible);
    command_buffer.add_component(spawned, render_style);
    command_buffer.add_component(spawned, terrain_revealer);
}