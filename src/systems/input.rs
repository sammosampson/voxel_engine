use crate::prelude::*;

#[system(for_each)]
pub fn mouse_input(
    input: &mut MouseInput,
    #[resource] event_channel: &mut EventChannel<SystemEvent>,
    #[resource] event_registration: &mut EventChannelRegistrar) {

    let timed_block = start_timed_block(CycleCounter::MouseInput);

    if let Some(last_current_action) = input.current_actions.last() {
        input.last_previous_action = *last_current_action;
    }
    
    input.current_actions = vec!();

    let mut last_current_action = input.last_previous_action;

    for event in read_events_using_registration(
        event_channel,
        event_registration,
        EventChannelRegistrationType::Input
    ) {
        let mut current_input = last_current_action.clone();
        current_input.wheel_movement = Vector2::default();

        match event {
            SystemEvent::PointerMoved { position} => {
                current_input.position = *position;
            },
            SystemEvent::MouseButtonAction { state, button} => {
                current_input.button = *button; 
                current_input.state = *state; 
            },
            SystemEvent::MouseWheelAction { delta } => {
                current_input.wheel_movement = *delta;
            }
            _ => {}
        }
        
        last_current_action = current_input;
        input.current_actions.push(current_input);
    }
    
    timed_block.stop();
}

#[system(simple)]
#[read_component(EditorVisibility)]
#[read_component(EditorVisible)]
pub fn editor_visibility_from_input(
    world: &legion::world::SubWorld,
    #[resource] event_channel: &mut EventChannel<SystemEvent>,
    #[resource] event_registration: &mut EventChannelRegistrar,
    command_buffer: &mut legion::systems::CommandBuffer
) {    
    let timed_block = start_timed_block(CycleCounter::EditorStateFromInput);

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

#[system(for_each)]
pub fn editor_state_from_input(
    editor_state: &mut EditorState,
    #[resource] event_channel: &mut EventChannel<SystemEvent>,
    #[resource] event_registration: &mut EventChannelRegistrar
) {    
    let timed_block = start_timed_block(CycleCounter::EditorStateFromInput);

    for editor_event in read_editor_event_using_registration(
        event_channel,
        event_registration,
        EventChannelRegistrationType::EditorState
    ) {
        match editor_event {
            EditorEvent::SetWindowVisibility(visible, window_name) =>
                set_editor_state(editor_state, visible, window_name),
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

fn set_editor_state(editor_state: &mut EditorState, visible: &bool, window_name: &String) {
    editor_state.set_window_visibility(*visible, window_name.clone())
}