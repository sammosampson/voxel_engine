use crate::prelude::*;

#[system]
#[read_component(WorldEntitySelected)]
#[write_component(Visible)]
#[write_component(RenderStyle)]
pub fn world_entity_selection_effects_entities(
    world: &mut SubWorld,
    command_buffer: &mut CommandBuffer
) {
    if query_selected_world_entities().iter(world).count() == 0 {
        return;
    }

    query_possibly_selected_world_entities()
        .iter_mut(world)
        .for_each(|(entity, visible, render_style, selected)| { 
            match selected {
                Some(_) => {
                    *visible = Visible(true);
                    *render_style = RenderStyle::Wireframe;
                    command_buffer.add_component(
                        *entity, 
                        ConstantRotation { 
                        x_revoloutions_per_second: 0.0,  
                        y_revoloutions_per_second: 0.25,  
                    });
                },
                None => {
                    *visible = Visible(false);
                    *render_style = RenderStyle::Fill;
                    command_buffer.remove_component::<ConstantRotation>(*entity); 
                },
            }
        });
}

fn query_selected_world_entities<'a>() -> legion::query::Query<&'a WorldEntitySelected> {
    <&WorldEntitySelected>::query()
}

fn query_possibly_selected_world_entities<'a>() -> legion::query::Query<(Entity, &'a mut Visible, &'a mut RenderStyle, Option<&'a WorldEntitySelected>)> {
    <(Entity, &mut Visible, &mut RenderStyle, Option<&WorldEntitySelected>)>::query()
}