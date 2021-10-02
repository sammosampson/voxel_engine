use crate::prelude::*;

#[system(for_each)]
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