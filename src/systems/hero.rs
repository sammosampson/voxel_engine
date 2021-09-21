use legion::*;
use crate::world;
use crate::hero;
use crate::position;
use crate::rotation;
use crate::math;
use crate::physics;
use crate::mesh;
use crate::rendering;
use crate::cameras;
use crate::input;
use crate::debug;
use crate::time;

pub fn add_hero_to_world(world: &mut legion::world::World) {
    world.push((
        hero::HeroSpawner::default(),
        position::Position::from(math::Vector4::position(0.0, 10.0, 0.0))
    ));
}

#[system(for_each)]
#[filter(component::<hero::HeroSpawner>())]
#[filter(!component::<hero::HeroSpawned>())]
pub fn spawn_hero(
    entity: &legion::Entity,
    position: &position::Position,
    command_buffer: &mut legion::systems::CommandBuffer
) {
    command_buffer.add_component(*entity, hero::HeroSpawned::default());

    let world_id = world::WorldEntityId::from("hero");
    let rotation = rotation::Rotation::default();
    let velocity = physics::Velocity::default();
    let heading = physics::Heading::from(math::Vector4::direction(0.0, 0.0, 1.0));
    let time = time::ElapsedTime::default();
    let mesh =  mesh::unit_cube(rendering::Colour::green());
    let input = input::MouseInput::default();
    let editor_visibility = debug::EditorVisibility::default();
    
    let attach_camera = cameras::AttachCamera {  
        offset_position: math::Vector4::position(0.0, 1.0, 0.0), 
        offset_direction: math::Vector4::direction(0.0, -0.2, 0.0)
    };

    let camera = cameras::Camera {  
        position:  math::Vector4::default(), 
        direction: math::Vector4::default(), 
        up: math::Vector4::up()
    };

    let window_size = rendering::WindowSize {
        size: math::Size2d::new(800.0, 600.0)
    };

    let visible = world::Visible(true);

    
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

    command_buffer.add_component(spawned, editor_visibility);
    command_buffer.add_component(spawned, attach_camera);
    command_buffer.add_component(spawned, camera);
    command_buffer.add_component(spawned, window_size);
    command_buffer.add_component(spawned, visible);
}

#[system(for_each)]
#[filter(!component::<debug::EditorVisible>())]
pub fn move_hero_from_mouse_action(
    input: &input::MouseInput,
    velocity: &mut physics::Velocity,
    heading: &mut physics::Heading,
    window_size: &rendering::WindowSize
) {
    let timed_block = debug::TimedBlock::start(debug::CycleCounter::MoveHeroFromMouseInput);

    let mut last_action = &input.last_previous_action;

    for next_action in &input.current_actions {
        if next_action.left_button_pressed() {
            velocity.increase_by(1.0);
        }
        if next_action.right_button_pressed() {
            velocity.increase_by(-1.0);
        }
        let delta = next_action.position - last_action.position;
        if !delta.zeroed() {
            heading.rotate_y_by(delta.x / window_size.size.width)
        }
        last_action = next_action;
    }

    timed_block.stop();
}