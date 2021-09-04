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

#[system(for_each)]
pub fn editor_state_from_input(
    editor_state: &mut debug::Editor,
    #[resource] event_channel: &mut shrev::EventChannel<events::SystemEvent>,
    #[resource] event_registration: &mut events::EventChannelRegistrar
) {    
    let timed_block = debug::TimedBlock::start(debug::CycleCounter::EditorStateFromInput);

    for event in event_channel.read(event_registration.lookup_registration(events::EventChannelRegistrationType::EditorState)) {
        match event {
            events::SystemEvent::KeyboardAction { state, button} => {
                if button.is_pressed(glium::glutin::event::VirtualKeyCode::F12, state) {
                    editor_state.editor_visible = !editor_state.editor_visible;
                }
            },
            _ => {}
        }
    }
    
    timed_block.stop();
}