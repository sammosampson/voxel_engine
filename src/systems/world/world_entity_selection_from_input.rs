use crate::prelude::*;

#[system(simple)]
#[read_component(WorldEntityId)]
#[read_component(WorldEntitySelected)]
pub fn world_entity_selection_from_input(
    world: &SubWorld,
    #[resource] event_channel: &mut EventChannel<SystemEvent>,
    #[resource] event_registration: &mut EventChannelRegistrar,
    command_buffer: &mut CommandBuffer
) {
    let timed_block = start_timed_block(CycleCounter::WorldEntitySelectionFromInput);
    
    for editor_event in read_editor_event_using_registration(
        event_channel,
        event_registration,
        EventChannelRegistrationType::WorldEntitySelectionFromInput
    ) {
        match editor_event {
            EditorEvent::EntityNodeSelect(selected_id) => {
                for (entity, id, selected) in query_possibly_selected_world_entities().iter(world) {
                    toggle_world_entity_selection(selected, selected_id, id, command_buffer, entity);
                }
            },
            _ => {}
        }
    }
 
    timed_block.stop();
}

fn query_possibly_selected_world_entities<'a>() -> legion::query::Query<(Entity, &'a WorldEntityId, TryRead<WorldEntitySelected>)> {
    <(Entity, &WorldEntityId, TryRead<WorldEntitySelected>)>::query()
}

fn toggle_world_entity_selection(
    selected: Option<&WorldEntitySelected>,
    selected_id: &String,
    id: &WorldEntityId,
    command_buffer: &mut CommandBuffer,
    entity: &Entity
) {
    if selected.is_none() && *selected_id == id.name {
        command_buffer.add_component(*entity, WorldEntitySelected);
    } else {
        command_buffer.remove_component::<WorldEntitySelected>(*entity);
    }
}