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
    let timed_block = debug::TimedBlock::start(debug::CycleCounter::ApplyHeadingAndVelocityToPosition);
    position.value  = position.value + (heading.value * (velocity.value * time.seconds));
    timed_block.stop();
}