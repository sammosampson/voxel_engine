use crate::prelude::*;

#[system(for_each)]
pub fn set_render_style(
    entity: &Entity,
    style: &RenderStyle, 
    #[resource] graph: &mut WorldRenderGraph
) {
    graph.find(entity).unwrap().set_render_style(*style);
}