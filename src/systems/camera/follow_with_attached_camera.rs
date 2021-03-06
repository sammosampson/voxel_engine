use crate::prelude::*;

#[system(for_each)]
pub fn follow_with_attached_camera(
    camera: &mut Camera,
    attach_camera: &AttachCamera,
    position: &Position,
    heading: &Heading
) {
    let timed_block = start_timed_block(CycleCounter::FollowWithAttachedCamera);
    camera.position = position.0 + attach_camera.offset_position;
    camera.direction = heading.value + attach_camera.offset_direction;

    timed_block.stop();
}