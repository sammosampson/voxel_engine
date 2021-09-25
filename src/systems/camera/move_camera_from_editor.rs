use crate::prelude::*;

#[system(for_each)]
pub fn move_camera_from_editor(
    camera: &mut Camera,
    #[resource] event_channel: &mut shrev::EventChannel<SystemEvent>,
    #[resource] event_registration: &mut EventChannelRegistrar
) {
    let timed_block = start_timed_block(CycleCounter::MoveCameraFromEditor);

    for editor_event in read_editor_event_using_registration(
        event_channel,
        event_registration,
        EventChannelRegistrationType::CameraMovementFromEditor
    ) {
        match editor_event {
            EditorEvent::CameraPositionChanged(position) => camera.position = *position,
            EditorEvent::CameraDirectionChanged(direction) => camera.direction = *direction,
            EditorEvent::CameraUpChanged(up) => camera.up = *up,
            _ => {}
        }
    }
    
    timed_block.stop();
}