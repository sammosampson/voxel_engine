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