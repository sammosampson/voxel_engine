use legion::*;
use crate::events;
use crate::input;
use crate::math;
use crate::debug;

#[system(for_each)]
pub fn mouse_input(
    input: &mut input::MouseInput,
    #[resource] event_channel: &mut shrev::EventChannel<events::SystemEvent>,
    #[resource] event_registration: &mut events::EventChannelRegistrar) {

    let timed_block = debug::TimedBlock::start(debug::CycleCounter::MouseInput);

    if let Some(last_current_action) = input.current_actions.last() {
        input.last_previous_action = *last_current_action;
    }
    
    input.current_actions = vec!();

    let mut last_current_action = input.last_previous_action;

    for event in event_channel.read(event_registration.lookup_registration(events::EventChannelRegistrationType::Input)) {
        let mut current_input = last_current_action.clone();
        current_input.wheel_movement = math::Vector2::default();

        match event {
            events::SystemEvent::PointerMoved { position} => {
                current_input.position = *position;
            },
            events::SystemEvent::MouseButtonAction { state, button} => {
                current_input.button = *button; 
                current_input.state = *state; 
            },
            events::SystemEvent::MouseWheelAction { delta } => {
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
#[read_component(debug::EditorVisibility)]
#[read_component(debug::EditorVisible)]
pub fn editor_visibility_from_input(
    world: &legion::world::SubWorld,
    #[resource] event_channel: &mut shrev::EventChannel<events::SystemEvent>,
    #[resource] event_registration: &mut events::EventChannelRegistrar,
    command_buffer: &mut legion::systems::CommandBuffer
) {    
    let timed_block = debug::TimedBlock::start(debug::CycleCounter::EditorStateFromInput);

    for event in event_channel.read(lookup_visibility_registration(event_registration)) {
        match event {
            events::SystemEvent::KeyboardAction { state, button} => {
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
    editor_state: &mut debug::EditorState,
    #[resource] event_channel: &mut shrev::EventChannel<events::SystemEvent>,
    #[resource] event_registration: &mut events::EventChannelRegistrar
) {    
    let timed_block = debug::TimedBlock::start(debug::CycleCounter::EditorStateFromInput);

    for event in event_channel.read(lookup_state_registration(event_registration)) {
        match event {
            events::SystemEvent::EditorChange(editor_event) => {
                match editor_event {
                    events::EditorEvent::SetWindowVisibility(visible, window_name) =>
                        set_editor_state(editor_state, visible, window_name),
                    _ => {}
                }
            }
            _ => {}
        }
    }
    
    timed_block.stop();
}

fn lookup_visibility_registration(event_registration: &mut events::EventChannelRegistrar
) -> &mut shrev::ReaderId<events::SystemEvent> {
    event_registration.lookup_registration(events::EventChannelRegistrationType::EditorVisibility)
}

fn is_editor_toggle_button_pressed(button: &input::KeyboardButton, state: &input::InputState
) -> bool {
    button.is_pressed(glium::glutin::event::VirtualKeyCode::F12, state)
}

fn toggle_editor_visibility(world: &world::SubWorld, command_buffer: &mut systems::CommandBuffer
) {
    let mut query = <legion::Entity>
        ::query()
        .filter(component::<debug::EditorVisibility>() & component::<debug::EditorVisible>());

    for entity in query.iter(world) {
        command_buffer.remove_component::<debug::EditorVisible>(*entity);
    }

    let mut query = <legion::Entity>
        ::query()
        .filter(component::<debug::EditorVisibility>() & !component::<debug::EditorVisible>());

    for entity in query.iter(world) {
        command_buffer.add_component(*entity, debug::EditorVisible::default());
    }
}

fn set_editor_state(editor_state: &mut debug::EditorState, visible: &bool, window_name: &String) {
    editor_state.set_window_visibility(*visible, window_name.clone())
}

fn lookup_state_registration(event_registration: &mut events::EventChannelRegistrar
) -> &mut shrev::ReaderId<events::SystemEvent> {
    event_registration.lookup_registration(events::EventChannelRegistrationType::EditorState)
}