use legion::*;
use crate::prelude::*;

#[system(for_each)]
pub fn apply_heading_and_velocity_to_position(
    velocity: &Velocity,
    heading: &Heading,
    position: &mut Position,
    last_position: &mut LastPosition,
    time: &ElapsedTime
) {
    let timed_block = start_timed_block(CycleCounter::ApplyHeadingAndVelocityToPosition);
    *last_position = LastPosition::from(*position);
    *position = *position + (heading.value * (velocity.value * time.seconds));
    timed_block.stop();
}