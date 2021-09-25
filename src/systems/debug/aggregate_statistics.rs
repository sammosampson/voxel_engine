use crate::prelude::*;

#[system(simple)]
#[read_component(BlockPerformanceMeasurement)]
#[write_component(BlockPerformanceMeasurement)]
pub fn aggregate_statistics(
    world: &mut SubWorld
) {  
    let timed_block = start_timed_block(CycleCounter::AggregateStatistics);
    
    let total_cycles = read_statistics(CycleCounterMeasurementsChannelReaders::AggregateTotal)
        .fold(0, |running_total: u64, measurement| {
            if let Some(running_total) = running_total.checked_add(measurement.cycles) {
                return running_total;
            }
            running_total
        });

    
    let aggregated_measurements = read_statistics(CycleCounterMeasurementsChannelReaders::Aggregate)
        .into_grouping_map_by(|measurement|&measurement.counter)
        .aggregate(|aggregated_measurement, _counter, cycles_measurement| {
            
            if let Some(aggregated_measurement) = aggregated_measurement {
                Some(aggregated_measurement + cycles_measurement)
            } else {
                Some(BlockPerformanceMeasurement::from((cycles_measurement, total_cycles)))
            }
        });

    let mut query = <&mut BlockPerformanceMeasurement>::query();
   
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