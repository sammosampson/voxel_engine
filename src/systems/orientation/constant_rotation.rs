
use crate::prelude::*;

#[system(par_for_each)]
pub fn constant_rotation(
    time: &ElapsedTime,
    speed: &ConstantRotation,
    rotation: &mut Rotation
) {   
    let timed_block = start_timed_block(CycleCounter::ConstantRotation);
    rotation.x = calculate_rotation(rotation.x, speed.x_revoloutions_per_second, time);
    rotation.y = calculate_rotation(rotation.y, speed.y_revoloutions_per_second, time);
    timed_block.stop();
}

fn calculate_rotation(current_rotation: f32, speed:f32, time: &ElapsedTime) -> f32 {
    let two_pi = PI * 2.0;
    let elapsed_revolutions = time.seconds as f32 * speed;
    let total_radians = (elapsed_revolutions * two_pi) + current_rotation;
    total_radians % two_pi
}