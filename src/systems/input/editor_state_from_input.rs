use crate::prelude::*;

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

fn set_editor_state(editor_state: &mut EditorState, visible: &bool, window_name: &String) {
    editor_state.set_window_visibility(*visible, window_name.clone())
}