use crate::prelude::*;

#[system]
#[filter(component::<EditorState>())]
pub fn editor_visibility_from_input(
    #[resource] event_channel: &mut EventChannel<SystemEvent>,
    #[resource] event_registration: &mut EventChannelRegistrar,
    #[resource] game_state: &mut GameState
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
                    toggle_editor_visibility(game_state);
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

fn toggle_editor_visibility(state: &mut GameState) {
    match state {
        GameState::Playing => *state = GameState::Editing,
        GameState::Editing => *state = GameState::Playing,
    }
}