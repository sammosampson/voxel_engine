use crate::prelude::*;

pub fn add_debugging_to_world(world: &mut legion::World) {
    let all_counters = CycleCounter::get_all();

    for counter in all_counters {
        world.push((BlockPerformanceMeasurement::from(counter),));
    }

    world.push((Debug::default(), ElapsedTime::default()));
}