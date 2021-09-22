use crate::prelude::*;

#[system(for_each)]
#[filter(!component::<EditorVisible>())]
pub fn follow_with_attached_camera(
    camera: &mut Camera,
    attach_camera: &AttachCamera,
    position: &Position,
    heading: &Heading
) {
    let timed_block = start_timed_block(CycleCounter::FollowWithAttachedCamera);
    camera.position = position.value + attach_camera.offset_position;
    camera.direction = heading.value + attach_camera.offset_direction;

    timed_block.stop();
}

#[system(for_each)]
pub fn set_camera_to_render_view_matrix(
    camera: &Camera, 
    #[resource] graph: &mut WorldRenderGraph
) {
    let timed_block = start_timed_block(CycleCounter::SetCameraToRenderViewMatrix);

    graph.view = Matrix4x4::view(
        camera.position, 
        camera.direction, 
        camera.up
    );

    timed_block.stop();
}

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

#[system(for_each)]
#[filter(component::<EditorVisible>())]
pub fn move_editor_camera_from_mouse_action(
    input: &MouseInput,
    camera: &mut Camera,
    window_size: &WindowSize
) {
    let timed_block = start_timed_block(CycleCounter::MoveEditorCameraFromMouseInput);

    let mut last_action = &input.last_previous_action;

    for next_action in &input.current_actions {
        if last_action.middle_button_pressed() && next_action.middle_button_pressed_or_released() {
            let delta = next_action.position - last_action.position;
            if delta.zeroed() {
                continue;
            }

            let translation = Vector4::position(
                -(delta.x / window_size.size.width) * 10.0, 
                (delta.y / window_size.size.height) * 10.0,
                0.0);

            camera.position = Matrix4x4::translation(translation) * camera.position;            
        }

        if last_action.right_button_pressed() && next_action.right_button_pressed_or_released() {
            let delta = next_action.position - last_action.position;
            
            if delta.zeroed() {
                continue;
            }

            camera.direction = 
                Matrix4x4::x_rotation(-delta.y / window_size.size.height) 
                * Matrix4x4::y_rotation(delta.x / window_size.size.width) 
                * camera.direction;
        }

        if next_action.mouse_wheel_moved() {
            camera.position.z = camera.position.z - next_action.wheel_movement.y;
        }

        last_action = next_action;
    }

    timed_block.stop();
}