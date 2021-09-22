use crate::prelude::*;

#[system(for_each)]
pub fn process_window_events(
    window_size: &mut WindowSize,
    #[resource] event_channel: &mut EventChannel<SystemEvent>,
    #[resource] event_registration: &mut EventChannelRegistrar,
    #[resource] exit_state_notifier: &mut ExitStateNotifier
) {
    let timed_block = start_timed_block(CycleCounter::StoreWindowInformation);

    for event in read_events_using_registration(
        event_channel,
        event_registration,
        EventChannelRegistrationType::Window
    ) {
        match event {
            SystemEvent::Resized(size) => window_size.size = *size,
            SystemEvent::CloseRequested => exit_state_notifier.should_exit = true,
            _ => {}
        }
    }

    timed_block.stop();
}
