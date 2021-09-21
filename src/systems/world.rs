use legion::*;
use crate::world;
use crate::rendering;
use crate::events;
use crate::debug;

#[system(simple)]
#[read_component(world::WorldEntityId)]
#[read_component(world::WorldEntitySelected)]
pub fn world_entity_selection_from_input(
    world: &legion::world::SubWorld,
    #[resource] event_channel: &mut shrev::EventChannel<events::SystemEvent>,
    #[resource] event_registration: &mut events::EventChannelRegistrar,
    command_buffer: &mut legion::systems::CommandBuffer
) {
    let timed_block = debug::TimedBlock::start(debug::CycleCounter::WorldEntitySelectionFromInput);

    for event in event_channel.read(event_registration.lookup_registration(events::EventChannelRegistrationType::WorldEntitySelectionFromInput)) {
        match event {
            events::SystemEvent::EditorChange(editor_event) => {
                match editor_event {
                    events::EditorEvent::EntityNodeSelect(id) => 
                        toggle_world_entity_selection(id, world, command_buffer),
                    _ => {}
                }
            },
            _ => {}
        }
    }
    
    timed_block.stop();
}

fn toggle_world_entity_selection(
    selected_id: &str,
    world: &legion::world::SubWorld,
    command_buffer: &mut legion::systems::CommandBuffer
) {                   
    let mut query = <(legion::Entity, &world::WorldEntityId, TryRead<world::WorldEntitySelected>)>
        ::query();

    for (entity, id, selected) in query.iter(world) {
        if selected.is_none() && selected_id == id.name {
            command_buffer.add_component(*entity, world::WorldEntitySelected::default());
        } else {
            command_buffer.remove_component::<world::WorldEntitySelected>(*entity);
        }
    }
}

#[system]
#[read_component(world::WorldEntitySelected)]
#[write_component(world::Visible)]
pub fn world_entity_selection_hides_all_other_entities(world: &mut legion::world::SubWorld) {
    if <&world::WorldEntitySelected>::query().iter(world).count() == 0 {
        return;
    }

    <(&mut world::Visible, Option<&world::WorldEntitySelected>)>::query()
        .iter_mut(world)
        .for_each(|(visible, selected)| *visible = world::Visible(selected.is_some()));
}


#[system(for_each)]
pub fn set_world_visibility(
    entity: &Entity,
    visible: &world::Visible, 
    #[resource] graph: &mut rendering::WorldRenderGraph
) {
    graph.find(entity).unwrap().set_visibility(visible.0);
}