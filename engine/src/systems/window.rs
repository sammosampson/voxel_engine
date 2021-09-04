
use legion::*;
use voxel_engine_app::ExitStateNotifier;
use voxel_engine_app::events;
use voxel_engine_app::rendering;
use voxel_engine_app::debug;

#[system(for_each)]
pub fn process_window_events(
    window_size: &mut rendering::WindowSize,
    #[resource] event_channel: &mut shrev::EventChannel<events::SystemEvent>,
    #[resource] event_registration: &mut events::EventChannelRegistrar,
    #[resource] exit_state_notifier: &mut ExitStateNotifier
) {
    let timed_block = debug::TimedBlock::start(debug::CycleCounter::StoreWindowInformation);

    for event in event_channel.read(event_registration.lookup_registration(events::EventChannelRegistrationType::Window)) {
        match event {
            events::SystemEvent::Resized(size) => window_size.size = *size,
            events::SystemEvent::CloseRequested => exit_state_notifier.should_exit = true,
            _ => {}
        }
    }

    timed_block.stop();
}
