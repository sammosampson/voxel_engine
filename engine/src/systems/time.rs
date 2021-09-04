
use std::time::Instant;
use legion::*;
use legion::world::*;
use voxel_engine_app::time;

#[system(simple)]
#[read_component(time::ElapsedTime)]
#[write_component(time::ElapsedTime)]
pub fn calculate_elapsed_time(
    #[state] time: &mut Instant,
    world: &mut SubWorld
) {
    let now = Instant::now();
    let elapsed_time = now - *time;

    let mut query = <&mut time::ElapsedTime>::query();

    for time in query.iter_mut(world) {
        time.seconds = elapsed_time.as_secs_f32();
    }

    *time = now;
}