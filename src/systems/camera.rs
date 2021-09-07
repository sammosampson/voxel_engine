use legion::*;
use crate::cameras;
use crate::rendering;
use crate::math;
use crate::events;
use crate::input;
use crate::debug;
use crate::position;
use crate::physics;

#[system(for_each)]
#[filter(!component::<debug::EditorVisible>())]
pub fn follow_with_attached_camera(
    camera: &mut cameras::Camera,
    attach_camera: &cameras::AttachCamera,
    position: &position::Position,
    heading: &physics::Heading
) {
    let timed_block = debug::TimedBlock::start(debug::CycleCounter::FollowWithAttachedCamera);
    camera.position = position.value + attach_camera.offset_position;
    camera.direction = heading.value + attach_camera.offset_direction;

    timed_block.stop();
}

#[system(for_each)]
pub fn set_camera_to_render_view_matrix(
    camera: &cameras::Camera, 
    #[resource] graph: &mut rendering::WorldRenderGraph
) {
    let timed_block = debug::TimedBlock::start(debug::CycleCounter::SetCameraToRenderViewMatrix);

    graph.view = math::Matrix4x4::view(
        camera.position, 
        camera.direction, 
        camera.up
    );

    timed_block.stop();
}

#[system(for_each)]
pub fn move_camera_from_editor(
    camera: &mut cameras::Camera,
    #[resource] event_channel: &mut shrev::EventChannel<events::SystemEvent>,
    #[resource] event_registration: &mut events::EventChannelRegistrar
) {
    let timed_block = debug::TimedBlock::start(debug::CycleCounter::MoveCameraFromEditor);

    for event in event_channel.read(event_registration.lookup_registration(events::EventChannelRegistrationType::CameraMovementFromEditor)) {
        match event {
            events::SystemEvent::EditorChange(editor_event) => {
                match editor_event {
                    events::EditorEvent::CameraPositionChanged(position) => camera.position = *position,
                    events::EditorEvent::CameraDirectionChanged(direction) => camera.direction = *direction,
                    events::EditorEvent::CameraUpChanged(up) => camera.up = *up,
                    _ => {}
                }
            },
            _ => {}
        }
    }
    
    timed_block.stop();
}

#[system(for_each)]
#[filter(component::<debug::EditorVisible>())]
pub fn move_editor_camera_from_mouse_action(
    input: &input::MouseInput,
    camera: &mut cameras::Camera,
    window_size: &rendering::WindowSize
) {
    let timed_block = debug::TimedBlock::start(debug::CycleCounter::MoveEditorCameraFromMouseInput);

    let mut last_action = &input.last_previous_action;

    for next_action in &input.current_actions {
        if last_action.middle_button_pressed() && next_action.middle_button_pressed_or_released() {
            let delta = next_action.position - last_action.position;
            if delta.zeroed() {
                continue;
            }

            let translation = math::Vector4::position(
                -(delta.x / window_size.size.width) * 10.0, 
                (delta.y / window_size.size.height) * 10.0,
                0.0);

            camera.position = math::Matrix4x4::translation(translation) * camera.position;            
        }

        if last_action.right_button_pressed() && next_action.right_button_pressed_or_released() {
            let delta = next_action.position - last_action.position;
            
            if delta.zeroed() {
                continue;
            }

            camera.direction = 
                math::Matrix4x4::x_rotation(-delta.y / window_size.size.height) 
                * math::Matrix4x4::y_rotation(delta.x / window_size.size.width) 
                * camera.direction;
        }

        if next_action.mouse_wheel_moved() {
            camera.position.z = camera.position.z - next_action.wheel_movement.y;
        }

        last_action = next_action;
    }

    timed_block.stop();
}