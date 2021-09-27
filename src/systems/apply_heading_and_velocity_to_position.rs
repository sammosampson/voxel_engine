use legion::*;
use crate::position;
use crate::physics;
use crate::debug;
use crate::time;

#[system(for_each)]
pub fn apply_heading_and_velocity_to_position(
    velocity: &physics::Velocity,
    heading: &physics::Heading,
    position: &mut position::Position,
    time: &time::ElapsedTime
) {
    let timed_block = debug::start_timed_block(debug::CycleCounter::ApplyHeadingAndVelocityToPosition);
    position.0 = position.0 + (heading.value * (velocity.value * time.seconds));
    timed_block.stop();
}