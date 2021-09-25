use crate::prelude::*;

#[system(simple)]
#[read_component(EditorVisibility)]
#[read_component(EditorVisible)]
pub fn editor_visibility_from_input(
    world: &legion::world::SubWorld,
    #[resource] event_channel: &mut EventChannel<SystemEvent>,
    #[resource] event_registration: &mut EventChannelRegistrar,
    command_buffer: &mut legion::systems::CommandBuffer
) {    
    let timed_block = start_timed_block(CycleCounter::EditorVisibilityFromInput);

    for event in read_events_using_registration(
        event_channel,
        event_registration,
        EventChannelRegistrationType::EditorVisibility
    ) {
        match event {
            SystemEvent::KeyboardAction { state, button} => {
                if is_editor_toggle_button_pressed(button, state) {
                    toggle_editor_visibility(world, command_buffer);
                }
            },
            _ => {}
        }
    }
    
    timed_block.stop();
}

fn is_editor_toggle_button_pressed(button: &KeyboardButton, state: &InputState
) -> bool {
    button.is_pressed(glium::glutin::event::VirtualKeyCode::F12, state)
}

fn toggle_editor_visibility(world: &world::SubWorld, command_buffer: &mut systems::CommandBuffer
) {
    let mut query = <legion::Entity>
        ::query()
        .filter(component::<EditorVisibility>() & component::<EditorVisible>());

    for entity in query.iter(world) {
        command_buffer.remove_component::<EditorVisible>(*entity);
    }

    let mut query = <legion::Entity>
        ::query()
        .filter(component::<EditorVisibility>() & !component::<EditorVisible>());

    for entity in query.iter(world) {
        command_buffer.add_component(*entity, EditorVisible::default());
    }
}