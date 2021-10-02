use crate::prelude::*;

#[system(for_each)]
pub fn move_hero_from_mouse_action(
    input: &MouseInput,
    velocity: &mut Velocity,
    heading: &mut Heading,
    window_size: &WindowSize
) {
    let timed_block = start_timed_block(CycleCounter::MoveHeroFromMouseInput);

    let mut last_action = &input.last_previous_action;

    for next_action in &input.current_actions {
        if next_action.left_button_pressed() {
            velocity.increase_by(1.0);
        }
        if next_action.right_button_pressed() {
            velocity.increase_by(-1.0);
        }
        let delta = next_action.position - last_action.position;
        if !delta.zeroed() {
            heading.rotate_y_by(delta.x / window_size.size.width)
        }
        last_action = next_action;
    }

    timed_block.stop();
}