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
            EditorEvent::EntityNodeSelect(id) => 
                toggle_world_entity_selection(id, world, command_buffer),
            _ => {}
        }
    }
 
    timed_block.stop();
}

fn toggle_world_entity_selection(
    selected_id: &str,
    world: &SubWorld,
    command_buffer: &mut CommandBuffer
) {                   
    let mut query = <(Entity, &WorldEntityId, TryRead<WorldEntitySelected>)>
        ::query();

    for (entity, id, selected) in query.iter(world) {
        if selected.is_none() && selected_id == id.name {
            command_buffer.add_component(*entity, WorldEntitySelected);
        } else {
            command_buffer.remove_component::<WorldEntitySelected>(*entity);
        }
    }
}

#[system]
#[read_component(WorldEntitySelected)]
#[write_component(Visible)]
pub fn world_entity_selection_hides_all_other_entities(world: &mut SubWorld
) {
    if <&WorldEntitySelected>::query().iter(world).count() == 0 {
        return;
    }

    <(&mut Visible, Option<&WorldEntitySelected>)>::query()
        .iter_mut(world)
        .for_each(|(visible, selected)| *visible = Visible(selected.is_some()));
}


#[system(for_each)]
pub fn set_world_visibility(
    entity: &Entity,
    visible: &Visible, 
    #[resource] graph: &mut WorldRenderGraph
) {
    graph.find(entity).unwrap().set_visibility(visible.0);
}