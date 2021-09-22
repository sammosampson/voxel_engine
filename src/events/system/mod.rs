use crate::prelude::*;

mod channel;
mod wrapped;
mod looping;

pub use channel::*;
pub use wrapped::*;
pub use looping::*;

pub fn read_events_using_registration<'a>(
    event_channel: &'a mut EventChannel<SystemEvent>,
    event_registration: &mut EventChannelRegistrar,
    registration_type: EventChannelRegistrationType
) -> EventIterator<'a, SystemEvent> {
    event_channel.read(event_registration.lookup_registration(registration_type))
}

pub fn read_editor_event_using_registration<'a>(
    event_channel: &'a mut EventChannel<SystemEvent>,
    event_registration: &mut EventChannelRegistrar,
    registration_type: EventChannelRegistrationType
) -> impl Iterator<Item = &'a EditorEvent> + 'a {
    read_events_using_registration(event_channel, event_registration, registration_type)
        .filter(|event| { 
            match **event {
                SystemEvent::EditorChange(_) => true,
                _ => false
            }
        })
        .map(|event| { 
            match event {
                SystemEvent::EditorChange(editor_event) => editor_event,
                _ => &EditorEvent::None
            }
        })
}