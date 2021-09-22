
use legion::*;
use legion::world::*;
use itertools::*;
use crate::debug;
use crate::time;

pub fn add_debugging_to_world(world: &mut legion::World) {
    let all_counters = debug::CycleCounter::get_all();

    for counter in all_counters {
        world.push((debug::BlockPerformanceMeasurement::from(counter),));
    }

    world.push((
        debug::Debug::default(),
        time::ElapsedTime::default()
        )
    );
}

#[system(simple)]
#[read_component(debug::BlockPerformanceMeasurement)]
#[write_component(debug::BlockPerformanceMeasurement)]
pub fn aggregate_statistics(
    world: &mut SubWorld
) {  
    let timed_block = debug::start_timed_block(debug::CycleCounter::AggregateStatistics);
    
    let total_cycles = debug::read_statistics(debug::CycleCounterMeasurementsChannelReaders::AggregateTotal)
        .fold(0, |running_total: u64, measurement| {
            if let Some(running_total) = running_total.checked_add(measurement.cycles) {
                return running_total;
            }
            running_total
        });

    
    let aggregated_measurements = debug::read_statistics(debug::CycleCounterMeasurementsChannelReaders::Aggregate)
        .into_grouping_map_by(|measurement|&measurement.counter)
        .aggregate(|aggregated_measurement, _counter, cycles_measurement| {
            
            if let Some(aggregated_measurement) = aggregated_measurement {
                Some(aggregated_measurement + cycles_measurement)
            } else {
                Some(debug::BlockPerformanceMeasurement::from((cycles_measurement, total_cycles)))
            }
        });

    let mut query = <&mut debug::BlockPerformanceMeasurement>::query();
   
    for measurement in query.iter_mut(world) {
        if let Some(new_measurement) = &aggregated_measurements.get(&measurement.counter) {
            measurement.counter = new_measurement.counter;
            measurement.cycles = new_measurement.cycles;
            measurement.total_cycles = new_measurement.total_cycles;
            measurement.hits = new_measurement.hits;
        } else {
            measurement.cycles = 0;
            measurement.total_cycles = 0;
            measurement.hits = 0;
        }
    }
    
    timed_block.stop();
}