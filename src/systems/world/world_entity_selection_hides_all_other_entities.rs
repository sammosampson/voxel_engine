use crate::prelude::*;

#[system]
#[read_component(WorldEntitySelected)]
#[write_component(Visible)]
pub fn world_entity_selection_hides_all_other_entities(world: &mut SubWorld) {
    if query_selected_world_entities().iter(world).count() == 0 {
        return;
    }

    query_possibly_selected_world_entities_visibility()
        .iter_mut(world)
        .for_each(|(visible, selected)| *visible = Visible(selected.is_some()));
}

fn query_selected_world_entities<'a>() -> legion::query::Query<&'a WorldEntitySelected> {
    <&WorldEntitySelected>::query()
}

fn query_possibly_selected_world_entities_visibility<'a>() -> legion::query::Query<(&'a mut Visible, Option<&'a WorldEntitySelected>)> {
    <(&mut Visible, Option<&WorldEntitySelected>)>::query()
}