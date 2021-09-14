use shrev::EventChannel;
use crate::events;

pub fn create_system_event_channel_registrar(channel: &mut shrev::EventChannel<events::SystemEvent>) -> EventChannelRegistrar {
    EventChannelRegistrar::new(channel)
}

pub fn create_system_event_channel() -> EventChannel::<events::SystemEvent> {
    EventChannel::<events::SystemEvent>::new()
}

pub fn create_system_event_producer() -> SystemEventProducer{
    SystemEventProducer::new()
}

pub enum EventChannelRegistrationType {
    Window,
    Input,
    CameraMovementFromEditor,
    EditorVisibility,
    EditorState,
    WorldEntitySelectionFromInput
}

pub struct EventChannelRegistrar {
    registrations: [shrev::ReaderId<events::SystemEvent>; 6]
}

impl EventChannelRegistrar {
    pub fn new(channel: &mut shrev::EventChannel<events::SystemEvent>) -> Self {
        Self {
            registrations: [
                channel.register_reader(),
                channel.register_reader(),
                channel.register_reader(),
                channel.register_reader(),
                channel.register_reader(),
                channel.register_reader()
            ]
        }
    }

    pub fn lookup_registration(&mut self, registration_type: EventChannelRegistrationType) -> &mut shrev::ReaderId<events::SystemEvent> {
        match registration_type {
            EventChannelRegistrationType::Window => &mut self.registrations[0],
            EventChannelRegistrationType::Input => &mut self.registrations[1],
            EventChannelRegistrationType::CameraMovementFromEditor => &mut self.registrations[2],
            EventChannelRegistrationType::EditorVisibility => &mut self.registrations[3],
            EventChannelRegistrationType::EditorState => &mut self.registrations[4],
            EventChannelRegistrationType::WorldEntitySelectionFromInput => &mut self.registrations[5],
        }
    }
}

pub struct SystemEventProducer {
    events: Vec<events::SystemEvent>,
}

impl SystemEventProducer {
    pub fn new() -> Self {
        Self {
            events: Vec::with_capacity(128),
        }
    }

    pub fn push(&mut self, to_push: events::SystemEvent) {
        self.events.push(to_push);
    }

    pub fn drain_to(&mut self, channel: &mut EventChannel::<events::SystemEvent>) {
        channel.drain_vec_write(&mut self.events);
    }
}